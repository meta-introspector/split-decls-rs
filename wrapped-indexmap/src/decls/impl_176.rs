macro_rules! deps {
    () => {
        ParUnion!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < T , S1 , S2 > Clone for ParUnion < '_ , T , S1 , S2 > { fn clone (& self) -> Self { ParUnion { .. * self } } }
    };
}

impl_176!()