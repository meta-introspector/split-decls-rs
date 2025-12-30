// Generated macro for build_string (function)
macro_rules! Depcrate_llvmbuild_string {
() => {
// Module: crate::llvm
// Provides: {"build_string"}
// Dependencies: {}
pub (crate) fn build_string (f : impl FnOnce (& RustString)) -> Result < String , FromUtf8Error > { String :: from_utf8 (RustString :: build_byte_buffer (f)) }
};
}
