macro_rules! deps {
    () => {
        ProcMacroLoc!();
        CrateRootModuleId!();
    };
}

macro_rules! macro_87 {
    () => {
        deps!();
        impl_loc ! (ProcMacroLoc , id : Fn , container : CrateRootModuleId) ;
    };
}

macro_87!()