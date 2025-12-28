macro_rules! deps {
    () => {
        ProcMacroLoadingError!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl fmt :: Display for ProcMacroLoadingError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ProcMacroLoadingError :: ExpectedProcMacroArtifact => { write ! (f , "proc-macro crate did not build proc-macro artifact") } ProcMacroLoadingError :: Disabled => write ! (f , "proc-macro expansion is disabled") , ProcMacroLoadingError :: FailedToBuild => write ! (f , "proc-macro failed to build") , ProcMacroLoadingError :: MissingDylibPath => { write ! (f , "proc-macro crate built but the dylib path is missing, this indicates a problem with your build system.") } ProcMacroLoadingError :: NotYetBuilt => write ! (f , "proc-macro not yet built") , ProcMacroLoadingError :: NoProcMacros => { write ! (f , "proc macro library has no proc macros") } ProcMacroLoadingError :: ProcMacroSrvError (msg) => { write ! (f , "proc macro server error: {msg}") } } } }
    };
}

impl_10!();