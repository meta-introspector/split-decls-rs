macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < K , V > ExactSizeIterator for Values < '_ , K , V > { # [inline] fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_97!()