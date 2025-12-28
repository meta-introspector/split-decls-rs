macro_rules! deps {
    () => {
        DebugLen!();
        Result!();
    };
}

macro_rules! impl_294 {
    () => {
        deps!();
        impl fmt :: Debug for DebugLen { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "...; {}" , self . 0) } }
    };
}

impl_294!();