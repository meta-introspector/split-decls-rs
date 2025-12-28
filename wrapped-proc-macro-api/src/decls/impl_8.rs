macro_rules! deps {
    () => {
        ProcMacro!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Eq for ProcMacro { }
    };
}

impl_8!()