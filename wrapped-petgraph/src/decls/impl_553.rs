macro_rules! deps {
    () => {
        NodeReferences!();
        IndexType!();
    };
}

macro_rules! impl_553 {
    () => {
        deps!();
        impl < N , Ix > ExactSizeIterator for NodeReferences < '_ , N , Ix > where Ix : IndexType { }
    };
}

impl_553!()