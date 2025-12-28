macro_rules! deps {
    () => {
        DebugByte!();
        Result!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        impl fmt :: Debug for DebugByte { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "0x{:02x}" , self . 0) } }
    };
}

impl_292!();