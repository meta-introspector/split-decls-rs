macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_848 {
    () => {
        deps!();
        impl < Fut : Unpin > ExactSizeIterator for Iter < '_ , Fut > { }
    };
}

impl_848!();