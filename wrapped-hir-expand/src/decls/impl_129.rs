macro_rules! deps {
    () => {
        Display!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl fmt :: Display for Display < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { display_fmt_path (self . db , self . path , f , self . edition) } }
    };
}

impl_129!();