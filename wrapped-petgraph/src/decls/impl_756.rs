macro_rules! deps {
    () => {
        IndexType!();
        EdgeReferences!();
    };
}

macro_rules! impl_756 {
    () => {
        deps!();
        impl < E , Ix > ExactSizeIterator for EdgeReferences < '_ , E , Ix > where Ix : IndexType { }
    };
}

impl_756!()