// Generated macro for twine_to_string (function)
macro_rules! Depcrate_llvmtwine_to_string {
() => {
// Module: crate::llvm
// Provides: {"twine_to_string"}
// Dependencies: {}
pub (crate) fn twine_to_string (tr : & Twine) -> String { unsafe { build_string (| s | LLVMRustWriteTwineToString (tr , s)) . expect ("got a non-UTF8 Twine from LLVM") } }
};
}
