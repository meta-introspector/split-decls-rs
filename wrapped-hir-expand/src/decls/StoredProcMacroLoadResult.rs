macro_rules! deps {
    () => {
        ProcMacro!();
    };
}

macro_rules! StoredProcMacroLoadResult {
    () => {
        deps!();
        type StoredProcMacroLoadResult = Result < Box < [ProcMacro] > , ProcMacroLoadingError > ;
    };
}

StoredProcMacroLoadResult!()