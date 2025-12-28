macro_rules! deps {
    () => {
        Events!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl fmt :: Debug for Events { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self) . finish () } }
    };
}

impl_58!();