macro_rules! deps {
    () => {
        Formatter!();
        Result!();
        Span!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Span { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "Span({:?}, {:?})" , self . start , self . end) } }
    };
}

impl_53!();