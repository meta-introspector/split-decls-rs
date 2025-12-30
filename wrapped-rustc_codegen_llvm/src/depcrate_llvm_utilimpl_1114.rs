// Generated macro for impl_1114 (impl)
macro_rules! Depcrate_llvm_utilimpl_1114 {
() => {
// Module: crate::llvm_util
// Provides: {"impl_1114"}
// Dependencies: {}
impl < 'a > IntoIterator for LLVMFeature < 'a > { type Item = & 'a str ; type IntoIter = impl Iterator < Item = & 'a str > ; fn into_iter (self) -> Self :: IntoIter { let dependencies = self . dependencies . into_iter () . map (| feat | feat . as_str ()) ; std :: iter :: once (self . llvm_feature_name) . chain (dependencies) } }
};
}
