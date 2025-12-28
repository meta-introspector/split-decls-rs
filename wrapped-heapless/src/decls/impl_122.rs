macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < K , V > Clone for Iter < '_ , K , V > { fn clone (& self) -> Self { Self { iter : self . iter . clone () , } } }
    };
}

impl_122!();