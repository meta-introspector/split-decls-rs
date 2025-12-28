macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl < K , V > ExactSizeIterator for Values < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_317!();