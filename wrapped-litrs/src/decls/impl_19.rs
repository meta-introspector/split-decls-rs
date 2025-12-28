macro_rules! deps {
    () => {
        BoolLit!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl fmt :: Display for BoolLit { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (self . as_str ()) } }
    };
}

impl_19!()