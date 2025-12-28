macro_rules! deps {
    () => {
        IterPinMut!();
    };
}

macro_rules! impl_842 {
    () => {
        deps!();
        impl < Fut > ExactSizeIterator for IterPinMut < '_ , Fut > { }
    };
}

impl_842!()