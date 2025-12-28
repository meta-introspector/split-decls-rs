macro_rules! deps {
    () => {
        AnyValueId!();
    };
}

macro_rules! impl_605 {
    () => {
        deps!();
        impl Eq for AnyValueId { }
    };
}

impl_605!();