macro_rules! deps {
    () => {
        ParSymmetricDifference!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < T , S1 , S2 > Clone for ParSymmetricDifference < '_ , T , S1 , S2 > { fn clone (& self) -> Self { ParSymmetricDifference { .. * self } } }
    };
}

impl_172!()