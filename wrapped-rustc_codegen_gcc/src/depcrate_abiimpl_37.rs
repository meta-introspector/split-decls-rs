// Generated macro for impl_37 (impl)
macro_rules! Depcrate_abiimpl_37 {
() => {
// Module: crate::abi
// Provides: {"impl_37"}
// Dependencies: {}
impl AbiBuilderMethods for Builder < '_ , '_ , '_ > { fn get_param (& mut self , index : usize) -> Self :: Value { let func = self . current_func () ; let param = func . get_param (index as i32) ; let on_stack = if let Some (on_stack_param_indices) = self . on_stack_function_params . borrow () . get (& func) { on_stack_param_indices . contains (& index) } else { false } ; if on_stack { param . to_lvalue () . get_address (None) } else { param . to_rvalue () } } }
};
}
