macro_rules! covmap_var_name {
    () => {
        pub (crate) fn covmap_var_name () -> CString { CString :: new (llvm :: build_byte_buffer (| s | unsafe { llvm :: LLVMRustCoverageWriteCovmapVarNameToString (s) ; })) . expect ("covmap variable name should not contain NUL") }
    };
}

covmap_var_name!();