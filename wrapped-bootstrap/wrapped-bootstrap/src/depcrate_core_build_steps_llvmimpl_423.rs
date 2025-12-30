// Generated macro for impl_423 (impl)
macro_rules! Depcrate_core_build_steps_llvmimpl_423 {
() => {
// Module: crate::core::build_steps::llvm
// Provides: {"impl_423"}
// Dependencies: {}
impl LlvmBuildStatus { pub fn should_build (& self) -> bool { match self { LlvmBuildStatus :: AlreadyBuilt (_) => false , LlvmBuildStatus :: ShouldBuild (_) => true , } } # [cfg (test)] pub fn llvm_result (& self) -> & LlvmResult { match self { LlvmBuildStatus :: AlreadyBuilt (res) => res , LlvmBuildStatus :: ShouldBuild (meta) => & meta . res , } } }
};
}
