macro_rules! deps {
    () => {
        IndexType!();
        NodeReferences!();
    };
}

macro_rules! impl_553 {
    () => {
        deps!();
        impl < N , Ix > ExactSizeIterator for NodeReferences < '_ , N , Ix > where Ix : IndexType { }
    };
}

impl_553!();