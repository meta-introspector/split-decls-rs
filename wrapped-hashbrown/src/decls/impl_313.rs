macro_rules! deps {
    () => {
        Keys!();
    };
}

macro_rules! impl_313 {
    () => {
        deps!();
        impl < K , V > ExactSizeIterator for Keys < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_313!();