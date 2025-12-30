// Generated macro for bitcode_section_name (function)
macro_rules! Depcrate_back_writebitcode_section_name {
() => {
// Module: crate::back::write
// Provides: {"bitcode_section_name"}
// Dependencies: {}
pub (crate) fn bitcode_section_name (cgcx : & CodegenContext < LlvmCodegenBackend >) -> & 'static CStr { if cgcx . target_is_like_darwin { c"__LLVM,__bitcode" } else if cgcx . target_is_like_aix { c".ipa" } else { c".llvmbc" } }
};
}
