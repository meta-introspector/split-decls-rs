macro_rules! deps {
    () => {
        ParDifference!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < T , S1 , S2 > Clone for ParDifference < '_ , T , S1 , S2 > { fn clone (& self) -> Self { ParDifference { .. * self } } }
    };
}

impl_164!()