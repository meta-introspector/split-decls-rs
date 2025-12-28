macro_rules! deps {
    () => {
        ProcMacro!();
    };
}

macro_rules! ProcMacroLoadResult {
    () => {
        deps!();
        pub type ProcMacroLoadResult = Result < Vec < ProcMacro > , ProcMacroLoadingError > ;
    };
}

ProcMacroLoadResult!()