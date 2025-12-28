macro_rules! deps {
    () => {
        ProcMacroLoadingError!();
        CrateBuilderId!();
    };
}

macro_rules! ProcMacroPaths {
    () => {
        deps!();
        pub type ProcMacroPaths = FxHashMap < CrateBuilderId , Result < (String , AbsPathBuf) , ProcMacroLoadingError > > ;
    };
}

ProcMacroPaths!();