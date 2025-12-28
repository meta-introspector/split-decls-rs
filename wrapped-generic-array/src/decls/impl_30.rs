macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < T : Ord , N : ArrayLength > Ord for GenericArray < T , N > { # [inline (always)] fn cmp (& self , other : & GenericArray < T , N >) -> Ordering { Ord :: cmp (self . as_slice () , other . as_slice ()) } }
    };
}

impl_30!()