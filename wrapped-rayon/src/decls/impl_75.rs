macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < T > Clone for Iter < '_ , T > { fn clone (& self) -> Self { Iter { inner : self . inner . clone () , } } }
    };
}

impl_75!()