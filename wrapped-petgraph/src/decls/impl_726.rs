macro_rules! deps {
    () => {
        IndexType!();
        NodeIndices!();
    };
}

macro_rules! impl_726 {
    () => {
        deps!();
        impl < Ix : IndexType > ExactSizeIterator for NodeIndices < Ix > { }
    };
}

impl_726!();