macro_rules! set_variable_sanitizer_attrs {
    () => {
        pub (crate) fn set_variable_sanitizer_attrs (llval : & Value , attrs : & CodegenFnAttrs) { if attrs . no_sanitize . contains (SanitizerSet :: ADDRESS) { unsafe { llvm :: LLVMRustSetNoSanitizeAddress (llval) } ; } if attrs . no_sanitize . contains (SanitizerSet :: HWADDRESS) { unsafe { llvm :: LLVMRustSetNoSanitizeHWAddress (llval) } ; } }
    };
}

set_variable_sanitizer_attrs!()