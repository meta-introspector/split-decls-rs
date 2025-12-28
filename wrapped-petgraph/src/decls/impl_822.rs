macro_rules! deps {
    () => {
        IndexType!();
        EdgeReference!();
    };
}

macro_rules! impl_822 {
    () => {
        deps!();
        impl < E , Ix : IndexType > Clone for EdgeReference < '_ , E , Ix > { fn clone (& self) -> Self { * self } }
    };
}

impl_822!();