macro_rules! deps {
    () => {
        UnboundedSenderInner!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < T > Unpin for UnboundedSenderInner < T > { }
    };
}

impl_27!()