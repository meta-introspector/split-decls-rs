macro_rules! deps {
    () => {
        IterPinRef!();
    };
}

macro_rules! impl_846 {
    () => {
        deps!();
        impl < Fut > ExactSizeIterator for IterPinRef < '_ , Fut > { }
    };
}

impl_846!();