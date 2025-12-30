// Generated macro for impl_205 (impl)
macro_rules! Depcrate_machinst_abiimpl_205 {
() => {
// Module: crate::machinst::abi
// Provides: {"impl_205"}
// Dependencies: {}
impl SigData { # [doc = " Get total stack space required for arguments."] pub fn sized_stack_arg_space (& self) -> i64 { self . sized_stack_arg_space . into () } # [doc = " Get total stack space required for return values."] pub fn sized_stack_ret_space (& self) -> i64 { self . sized_stack_ret_space . into () } # [doc = " Get calling convention used."] pub fn call_conv (& self) -> isa :: CallConv { self . call_conv } # [doc = " The index of the stack-return-value-area argument, if any."] pub fn stack_ret_arg (& self) -> Option < u16 > { self . stack_ret_arg } }
};
}
