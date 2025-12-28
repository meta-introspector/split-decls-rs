macro_rules! deps {
    () => {
        ProcMacro!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl Eq for ProcMacro { }
    };
}

impl_25!();