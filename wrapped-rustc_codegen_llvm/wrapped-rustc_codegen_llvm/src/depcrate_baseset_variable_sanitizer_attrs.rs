// Generated macro for set_variable_sanitizer_attrs (function)
macro_rules! Depcrate_baseset_variable_sanitizer_attrs {
() => {
// Module: crate::base
// Provides: {"set_variable_sanitizer_attrs"}
// Dependencies: {}
pub (crate) fn set_variable_sanitizer_attrs (llval : & Value , attrs : & CodegenFnAttrs) { if attrs . no_sanitize . contains (SanitizerSet :: ADDRESS) { unsafe { llvm :: LLVMRustSetNoSanitizeAddress (llval) } ; } if attrs . no_sanitize . contains (SanitizerSet :: HWADDRESS) { unsafe { llvm :: LLVMRustSetNoSanitizeHWAddress (llval) } ; } }
};
}
