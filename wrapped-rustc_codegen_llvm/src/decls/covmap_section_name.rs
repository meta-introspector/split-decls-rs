macro_rules! covmap_section_name {
    () => {
        pub (crate) fn covmap_section_name (llmod : & llvm :: Module) -> CString { CString :: new (llvm :: build_byte_buffer (| s | unsafe { llvm :: LLVMRustCoverageWriteCovmapSectionNameToString (llmod , s) ; })) . expect ("covmap section name should not contain NUL") }
    };
}

covmap_section_name!();