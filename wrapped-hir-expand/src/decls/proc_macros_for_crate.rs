macro_rules! deps {
    () => {
        CrateProcMacros!();
        ExpandDatabase!();
    };
}

macro_rules! proc_macros_for_crate {
    () => {
        deps!();
        pub (crate) fn proc_macros_for_crate (db : & dyn ExpandDatabase , krate : Crate ,) -> Option < Arc < CrateProcMacros > > { db . proc_macros () . get (krate) }
    };
}

proc_macros_for_crate!();