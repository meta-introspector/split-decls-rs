macro_rules! covfun_section_name {
    () => {
        pub (crate) fn covfun_section_name (llmod : & llvm :: Module) -> CString { CString :: new (llvm :: build_byte_buffer (| s | unsafe { llvm :: LLVMRustCoverageWriteCovfunSectionNameToString (llmod , s) ; })) . expect ("covfun section name should not contain NUL") }
    };
}

covfun_section_name!();