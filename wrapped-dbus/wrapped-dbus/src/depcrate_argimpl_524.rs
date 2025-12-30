// Generated macro for impl_524 (impl)
macro_rules! Depcrate_argimpl_524 {
() => {
// Module: crate::arg
// Provides: {"impl_524"}
// Dependencies: {}
impl TypeMismatchError { # [doc = " The ArgType we were trying to read, but failed"] pub fn expected_arg_type (& self) -> ArgType { self . expected } # [doc = " The ArgType we should have been trying to read, if we wanted the read to succeed"] pub fn found_arg_type (& self) -> ArgType { self . found } # [doc = " At what argument was the error found?"] # [doc = ""] # [doc = " Returns 0 for first argument, 1 for second argument, etc."] pub fn pos (& self) -> u32 { self . position } }
};
}
