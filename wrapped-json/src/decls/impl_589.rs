macro_rules! deps {
    () => {
        Fused!();
        StrRead!();
    };
}

macro_rules! impl_589 {
    () => {
        deps!();
        impl < 'a > Fused for StrRead < 'a > { }
    };
}

impl_589!()