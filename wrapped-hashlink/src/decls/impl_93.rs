macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < K , V > Clone for Values < '_ , K , V > { # [inline] fn clone (& self) -> Self { Values { inner : self . inner . clone () , } } }
    };
}

impl_93!();