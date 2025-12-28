macro_rules! deps {
    () => {
        CanInto!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < T > CanInto < T > for T where T : Clone { }
    };
}

impl_30!();