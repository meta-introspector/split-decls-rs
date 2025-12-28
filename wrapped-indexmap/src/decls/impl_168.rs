macro_rules! deps {
    () => {
        ParIntersection!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < T , S1 , S2 > Clone for ParIntersection < '_ , T , S1 , S2 > { fn clone (& self) -> Self { ParIntersection { .. * self } } }
    };
}

impl_168!();