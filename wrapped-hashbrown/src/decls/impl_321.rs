macro_rules! deps {
    () => {
        ValuesMut!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl < K , V > ExactSizeIterator for ValuesMut < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_321!()