use cairo_rs::{
	hint_processor::{
		builtin_hint_processor::hint_utils::get_integer_from_var_name,
		hint_processor_definition::HintReference,
		proxies::{exec_scopes_proxy::ExecutionScopesProxy, vm_proxy::VMProxy},
	},
	serde::deserialize_program::ApTracking,
	vm::errors::vm_errors::VirtualMachineError,
};
use lazy_static::lazy_static;
use std::{
	collections::HashMap,
	io::{stdout, BufWriter, Stdout, Write},
	sync::Mutex,
};

lazy_static! {
	pub static ref HINT_OUTPUT_BUFFER: Mutex<BufWriter<Stdout>> = {
		let stdout = stdout();
		Mutex::new(BufWriter::new(stdout))
	};
}

fn write_to_output_buffer(data: &[u8]) {
	HINT_OUTPUT_BUFFER.lock().unwrap().write_all(data).unwrap();
}

pub fn greater_than(
	vm_proxy: &mut VMProxy,
	_exec_scopes_proxy: &mut ExecutionScopesProxy,
	ids_data: &HashMap<String, HintReference>,
	ap_tracking: &ApTracking,
) -> Result<(), VirtualMachineError> {
	let a = get_integer_from_var_name("a", vm_proxy, ids_data, ap_tracking)?;
	let b = get_integer_from_var_name("b", vm_proxy, ids_data, ap_tracking)?;
	write_to_output_buffer(format!("{}\n", a > b).as_bytes());
	Ok(())
}