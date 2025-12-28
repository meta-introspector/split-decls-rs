macro_rules! deps {
    () => {
        Windows!();
    };
}

macro_rules! impl_1252 {
    () => {
        deps!();
        impl < T > Clone for Windows < '_ , T > { fn clone (& self) -> Self { Windows { .. * self } } }
    };
}

impl_1252!()