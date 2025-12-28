macro_rules! CodegenDiagnosticsStage {
    () => {
        # [doc = " In what context is a dignostic handler being attached to a codegen unit?"] pub (crate) enum CodegenDiagnosticsStage { # [doc = " Prelink optimization stage."] Opt , # [doc = " LTO/ThinLTO postlink optimization stage."] LTO , # [doc = " Code generation."] Codegen , }
    };
}

CodegenDiagnosticsStage!()