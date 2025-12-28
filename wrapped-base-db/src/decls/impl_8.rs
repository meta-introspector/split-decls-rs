macro_rules! deps {
    () => {
        ProcMacroLoadingError!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl ProcMacroLoadingError { pub fn is_hard_error (& self) -> bool { match self { ProcMacroLoadingError :: Disabled | ProcMacroLoadingError :: NotYetBuilt => false , ProcMacroLoadingError :: ExpectedProcMacroArtifact | ProcMacroLoadingError :: FailedToBuild | ProcMacroLoadingError :: MissingDylibPath | ProcMacroLoadingError :: NoProcMacros | ProcMacroLoadingError :: ProcMacroSrvError (_) => true , } } }
    };
}

impl_8!()