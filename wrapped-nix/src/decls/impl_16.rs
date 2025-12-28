macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl fmt :: Display for Errno { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{:?}: {}" , self , self . desc ()) } }
    };
}

impl_16!()