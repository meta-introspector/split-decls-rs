macro_rules! deps {
    () => {
        UnboundedReceiver!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < T > Unpin for UnboundedReceiver < T > { }
    };
}

impl_35!();