macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl core :: fmt :: Display for Error { fn fmt (& self , fmt : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let message = self . message () ; if message . is_empty () { core :: write ! (fmt , "{}" , self . code ()) } else { core :: write ! (fmt , "{} ({})" , message , self . code ()) } } }
    };
}

impl_73!();