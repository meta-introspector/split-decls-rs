macro_rules! deps {
    () => {
        Fused!();
        SliceRead!();
    };
}

macro_rules! impl_588 {
    () => {
        deps!();
        impl < 'a > Fused for SliceRead < 'a > { }
    };
}

impl_588!()