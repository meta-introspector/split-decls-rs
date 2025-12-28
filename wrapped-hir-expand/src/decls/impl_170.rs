macro_rules! deps {
    () => {
        ProcMacros!();
        CrateProcMacros!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl ProcMacros { fn get (& self , krate : Crate) -> Option < Arc < CrateProcMacros > > { self . 0 . get (& krate) . cloned () } }
    };
}

impl_170!();