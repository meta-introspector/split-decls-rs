// Generated macro for impl_31 (impl)
macro_rules! Depcrate_inputimpl_31 {
() => {
// Module: crate::input
// Provides: {"impl_31"}
// Dependencies: {}
impl ProcMacroLoadingError { pub fn is_hard_error (& self) -> bool { match self { ProcMacroLoadingError :: Disabled | ProcMacroLoadingError :: NotYetBuilt => false , ProcMacroLoadingError :: ExpectedProcMacroArtifact | ProcMacroLoadingError :: FailedToBuild | ProcMacroLoadingError :: MissingDylibPath | ProcMacroLoadingError :: NoProcMacros | ProcMacroLoadingError :: ProcMacroSrvError (_) => true , } } }
};
}
