macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl core :: fmt :: Display for TokenStream { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "{}" , self . as_str ()) } }
    };
}

impl_146!()