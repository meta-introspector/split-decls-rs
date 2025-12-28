macro_rules! ProcMacroLoadingError {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum ProcMacroLoadingError { Disabled , FailedToBuild , ExpectedProcMacroArtifact , MissingDylibPath , NotYetBuilt , NoProcMacros , ProcMacroSrvError (Box < str >) , }
    };
}

ProcMacroLoadingError!();