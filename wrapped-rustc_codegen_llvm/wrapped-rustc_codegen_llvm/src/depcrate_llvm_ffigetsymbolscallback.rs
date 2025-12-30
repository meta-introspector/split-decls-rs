// Generated macro for GetSymbolsCallback (type)
macro_rules! Depcrate_llvm_ffiGetSymbolsCallback {
() => {
// Module: crate::llvm::ffi
// Provides: {"GetSymbolsCallback"}
// Dependencies: {}
pub (crate) type GetSymbolsCallback = unsafe extern "C" fn (* mut c_void , * const c_char) -> * mut c_void ;
};
}
