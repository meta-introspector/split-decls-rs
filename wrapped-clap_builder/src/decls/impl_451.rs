macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! impl_451 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for Values < T > { }
    };
}

impl_451!()