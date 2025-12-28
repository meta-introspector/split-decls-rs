macro_rules! deps {
    () => {
        ZeroToken!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl fmt :: Debug for ZeroToken { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& (self . 0 as usize) , f) } }
    };
}

impl_149!();