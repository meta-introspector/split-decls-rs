macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < T > Clone for Iter < '_ , T > { fn clone (& self) -> Self { Iter { inner : self . inner . clone () , } } }
    };
}

impl_53!()