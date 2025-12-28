macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_1246 {
    () => {
        deps!();
        impl < T > Clone for Iter < '_ , T > { fn clone (& self) -> Self { Iter { .. * self } } }
    };
}

impl_1246!();