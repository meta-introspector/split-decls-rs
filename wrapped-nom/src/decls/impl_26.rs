macro_rules! deps {
    () => {
        ContextError!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < I > ContextError < I > for () { }
    };
}

impl_26!();