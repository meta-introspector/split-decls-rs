macro_rules! deps {
    () => {
        AccessError!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl fmt :: Debug for AccessError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("AccessError") . finish () } }
    };
}

impl_173!();