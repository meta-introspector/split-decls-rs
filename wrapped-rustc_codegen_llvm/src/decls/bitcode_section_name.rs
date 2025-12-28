macro_rules! deps {
    () => {
        LlvmCodegenBackend!();
    };
}

macro_rules! bitcode_section_name {
    () => {
        deps!();
        pub (crate) fn bitcode_section_name (cgcx : & CodegenContext < LlvmCodegenBackend >) -> & 'static CStr { if cgcx . target_is_like_darwin { c"__LLVM,__bitcode" } else if cgcx . target_is_like_aix { c".ipa" } else { c".llvmbc" } }
    };
}

bitcode_section_name!()