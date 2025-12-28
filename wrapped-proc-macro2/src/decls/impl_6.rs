macro_rules! deps {
    () => {
        ProcMacroAutoTraits!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl UnwindSafe for ProcMacroAutoTraits { }
    };
}

impl_6!()