macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_908 {
    () => {
        deps!();
        impl < St : Stream + Unpin > ExactSizeIterator for IntoIter < St > { }
    };
}

impl_908!()