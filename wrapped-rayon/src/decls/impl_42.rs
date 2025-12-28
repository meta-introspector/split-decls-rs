macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < K , V > Clone for Iter < '_ , K , V > { fn clone (& self) -> Self { Iter { inner : self . inner . clone () , } } }
    };
}

impl_42!()