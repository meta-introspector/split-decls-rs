// Generated macro for GccCodegenBackendOutput (struct)
macro_rules! Depcrate_core_build_steps_compileGccCodegenBackendOutput {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"GccCodegenBackendOutput"}
// Dependencies: {}
# [doc = " Output of the `compile::GccCodegenBackend` step."] # [doc = " It includes the path to the libgccjit library on which this backend depends."] # [derive (Clone)] pub struct GccCodegenBackendOutput { stamp : BuildStamp , gcc : GccOutput , }
};
}
