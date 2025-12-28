macro_rules! deps {
    () => {
        ProcMacroLoc!();
    };
}

macro_rules! macro_658 {
    () => {
        deps!();
        impl_intern ! (ProcMacroId , ProcMacroLoc , intern_proc_macro , lookup_intern_proc_macro) ;
    };
}

macro_658!();