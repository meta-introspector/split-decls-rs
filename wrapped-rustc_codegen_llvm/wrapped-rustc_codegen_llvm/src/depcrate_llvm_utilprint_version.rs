// Generated macro for print_version (function)
macro_rules! Depcrate_llvm_utilprint_version {
() => {
// Module: crate::llvm_util
// Provides: {"print_version"}
// Dependencies: {}
pub (crate) fn print_version () { let (major , minor , patch) = get_version () ; println ! ("LLVM version: {major}.{minor}.{patch}") ; }
};
}
