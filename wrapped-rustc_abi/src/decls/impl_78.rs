macro_rules! deps {
    () => {
        Align!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl fmt :: Debug for Align { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Align({} bytes)" , self . bytes ()) } }
    };
}

impl_78!()