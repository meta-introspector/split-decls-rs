macro_rules! deps {
    () => {
        NonMaxUsize!();
    };
}

macro_rules! impl_754 {
    () => {
        deps!();
        impl core :: fmt :: Debug for NonMaxUsize { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "{:?}" , self . get ()) } }
    };
}

impl_754!()