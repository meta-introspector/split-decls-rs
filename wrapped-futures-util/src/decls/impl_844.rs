macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_844 {
    () => {
        deps!();
        impl < Fut : Unpin > ExactSizeIterator for IterMut < '_ , Fut > { }
    };
}

impl_844!();