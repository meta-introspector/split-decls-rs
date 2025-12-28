macro_rules! deps {
    () => {
        NodeIndices!();
        IndexType!();
    };
}

macro_rules! impl_726 {
    () => {
        deps!();
        impl < Ix : IndexType > ExactSizeIterator for NodeIndices < Ix > { }
    };
}

impl_726!()