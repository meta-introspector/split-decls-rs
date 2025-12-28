macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < T > Unpin for Receiver < T > { }
    };
}

impl_78!()