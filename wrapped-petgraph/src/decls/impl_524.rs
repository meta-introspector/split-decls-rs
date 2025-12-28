macro_rules! deps {
    () => {
        EdgeReference!();
    };
}

macro_rules! impl_524 {
    () => {
        deps!();
        impl < E , Ty , Ix : Copy > Clone for EdgeReference < '_ , E , Ty , Ix > { fn clone (& self) -> Self { * self } }
    };
}

impl_524!();