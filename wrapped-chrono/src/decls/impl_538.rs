macro_rules! deps {
    () => {
        FixedOffset!();
        Offset!();
    };
}

macro_rules! impl_538 {
    () => {
        deps!();
        impl Offset for FixedOffset { fn fix (& self) -> FixedOffset { * self } }
    };
}

impl_538!()