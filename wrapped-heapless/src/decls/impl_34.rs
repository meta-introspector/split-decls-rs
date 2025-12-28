macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for IterMut < '_ , T > { }
    };
}

impl_34!();