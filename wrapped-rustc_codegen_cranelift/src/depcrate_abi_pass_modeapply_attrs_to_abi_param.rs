// Generated macro for apply_attrs_to_abi_param (function)
macro_rules! Depcrate_abi_pass_modeapply_attrs_to_abi_param {
() => {
// Module: crate::abi::pass_mode
// Provides: {"apply_attrs_to_abi_param"}
// Dependencies: {}
fn apply_attrs_to_abi_param (param : AbiParam , arg_attrs : ArgAttributes) -> AbiParam { match arg_attrs . arg_ext { RustcArgExtension :: None => param , RustcArgExtension :: Zext => param . uext () , RustcArgExtension :: Sext => param . sext () , } }
};
}
