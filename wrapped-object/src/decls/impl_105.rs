macro_rules! deps {
    () => {
        Result!();
        DebugLen!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl fmt :: Debug for DebugLen { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "...; {}" , self . 0) } }
    };
}

impl_105!()