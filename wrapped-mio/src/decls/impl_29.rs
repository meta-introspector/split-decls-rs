macro_rules! deps {
    () => {
        Registry!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl fmt :: Debug for Registry { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("Registry") . finish () } }
    };
}

impl_29!()