macro_rules! deps {
    () => {
        AssertKinds!();
        UnboundedSender!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl AssertKinds for UnboundedSender < u32 > { }
    };
}

impl_32!();