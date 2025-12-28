macro_rules! deps {
    () => {
        Frame!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl fmt :: Debug for Frame { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Frame") . field ("ip" , & self . ip ()) . field ("symbol_address" , & self . symbol_address ()) . finish () } }
    };
}

impl_6!();