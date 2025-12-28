macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_299 {
    () => {
        deps!();
        impl < K , V > ExactSizeIterator for Iter < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_299!();