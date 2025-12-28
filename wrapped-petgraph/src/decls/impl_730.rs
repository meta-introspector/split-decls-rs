macro_rules! deps {
    () => {
        IndexType!();
        EdgeIndices!();
    };
}

macro_rules! impl_730 {
    () => {
        deps!();
        impl < Ix : IndexType > ExactSizeIterator for EdgeIndices < Ix > { }
    };
}

impl_730!();