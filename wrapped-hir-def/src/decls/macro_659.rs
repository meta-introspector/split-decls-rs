macro_rules! deps {
    () => {
        ProcMacroLoc!();
        CrateRootModuleId!();
    };
}

macro_rules! macro_659 {
    () => {
        deps!();
        impl_loc ! (ProcMacroLoc , id : Fn , container : CrateRootModuleId) ;
    };
}

macro_659!()