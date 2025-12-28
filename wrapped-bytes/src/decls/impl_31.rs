macro_rules! deps {
    () => {
        IntoIter!();
        Buf!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < T : Buf > ExactSizeIterator for IntoIter < T > { }
    };
}

impl_31!()