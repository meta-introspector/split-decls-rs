macro_rules! deps {
    () => {
        EdgeReference!();
        IndexType!();
    };
}

macro_rules! impl_732 {
    () => {
        deps!();
        impl < E , Ix : IndexType > Clone for EdgeReference < '_ , E , Ix > { fn clone (& self) -> Self { * self } }
    };
}

impl_732!();