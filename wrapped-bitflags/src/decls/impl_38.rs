macro_rules! deps {
    () => {
        ImplementedByBitFlagsMacro!();
        Flags!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < B : Flags > ImplementedByBitFlagsMacro for B { }
    };
}

impl_38!();