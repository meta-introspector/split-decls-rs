macro_rules! deps {
    () => {
        SliceRead!();
        Fused!();
    };
}

macro_rules! impl_588 {
    () => {
        deps!();
        impl < 'a > Fused for SliceRead < 'a > { }
    };
}

impl_588!();