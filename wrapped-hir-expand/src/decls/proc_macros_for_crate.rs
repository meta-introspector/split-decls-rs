macro_rules! deps {
    () => {
        ExpandDatabase!();
        CrateProcMacros!();
    };
}

macro_rules! proc_macros_for_crate {
    () => {
        deps!();
        pub (crate) fn proc_macros_for_crate (db : & dyn ExpandDatabase , krate : Crate ,) -> Option < Arc < CrateProcMacros > > { db . proc_macros () . get (krate) }
    };
}

proc_macros_for_crate!()