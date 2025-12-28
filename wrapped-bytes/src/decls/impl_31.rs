macro_rules! deps {
    () => {
        Buf!();
        IntoIter!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < T : Buf > ExactSizeIterator for IntoIter < T > { }
    };
}

impl_31!();