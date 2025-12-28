macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_840 {
    () => {
        deps!();
        impl < Fut : Unpin > ExactSizeIterator for IntoIter < Fut > { }
    };
}

impl_840!();