macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! impl_503 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Encoding { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "Encoding {{ {} }}" , self . name) } }
    };
}

impl_503!()