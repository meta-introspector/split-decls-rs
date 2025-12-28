macro_rules! deps {
    () => {
        LayoutCx!();
    };
}

macro_rules! impl_662 {
    () => {
        deps!();
        impl < 'a > LayoutCx < 'a > { fn new (target : & 'a TargetDataLayout) -> Self { Self { calc : LayoutCalculator :: new (target) } } }
    };
}

impl_662!()