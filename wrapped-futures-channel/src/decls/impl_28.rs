macro_rules! deps {
    () => {
        BoundedSenderInner!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < T > Unpin for BoundedSenderInner < T > { }
    };
}

impl_28!()