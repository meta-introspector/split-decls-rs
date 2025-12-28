macro_rules! deps {
    () => {
        EdgeIndices!();
        IndexType!();
    };
}

macro_rules! impl_730 {
    () => {
        deps!();
        impl < Ix : IndexType > ExactSizeIterator for EdgeIndices < Ix > { }
    };
}

impl_730!()