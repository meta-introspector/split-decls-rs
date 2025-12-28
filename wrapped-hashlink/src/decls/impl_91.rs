macro_rules! deps {
    () => {
        Keys!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < K , V > ExactSizeIterator for Keys < '_ , K , V > { # [inline] fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_91!()