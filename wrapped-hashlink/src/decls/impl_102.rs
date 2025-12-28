macro_rules! deps {
    () => {
        ValuesMut!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < K , V > ExactSizeIterator for ValuesMut < '_ , K , V > { # [inline] fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_102!()