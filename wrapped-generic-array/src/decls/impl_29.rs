macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < T : PartialOrd , N : ArrayLength > PartialOrd for GenericArray < T , N > { # [inline (always)] fn partial_cmp (& self , other : & GenericArray < T , N >) -> Option < Ordering > { PartialOrd :: partial_cmp (self . as_slice () , other . as_slice ()) } }
    };
}

impl_29!()