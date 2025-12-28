macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl < K , V > ExactSizeIterator for IterMut < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_303!()