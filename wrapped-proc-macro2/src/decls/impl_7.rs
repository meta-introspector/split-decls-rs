macro_rules! deps {
    () => {
        ProcMacroAutoTraits!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl RefUnwindSafe for ProcMacroAutoTraits { }
    };
}

impl_7!();