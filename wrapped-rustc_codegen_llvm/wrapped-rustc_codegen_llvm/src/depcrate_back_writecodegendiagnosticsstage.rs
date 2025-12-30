// Generated macro for CodegenDiagnosticsStage (enum)
macro_rules! Depcrate_back_writeCodegenDiagnosticsStage {
() => {
// Module: crate::back::write
// Provides: {"CodegenDiagnosticsStage"}
// Dependencies: {}
# [doc = " In what context is a dignostic handler being attached to a codegen unit?"] pub (crate) enum CodegenDiagnosticsStage { # [doc = " Prelink optimization stage."] Opt , # [doc = " LTO/ThinLTO postlink optimization stage."] LTO , # [doc = " Code generation."] Codegen , }
};
}
