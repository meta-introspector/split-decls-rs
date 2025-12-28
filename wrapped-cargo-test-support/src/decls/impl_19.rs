macro_rules! deps {
    () => {
        WildStr!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl fmt :: Debug for WildStr < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:?}" , self . line) } }
    };
}

impl_19!()