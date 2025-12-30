// Generated macro for impl_1113 (impl)
macro_rules! Depcrate_llvm_utilimpl_1113 {
() => {
// Module: crate::llvm_util
// Provides: {"impl_1113"}
// Dependencies: {}
impl < 'a > LLVMFeature < 'a > { fn new (llvm_feature_name : & 'a str) -> Self { Self { llvm_feature_name , dependencies : SmallVec :: new () } } fn with_dependencies (llvm_feature_name : & 'a str , dependencies : SmallVec < [TargetFeatureFoldStrength < 'a > ; 1] > ,) -> Self { Self { llvm_feature_name , dependencies } } }
};
}
