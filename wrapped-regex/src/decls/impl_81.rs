macro_rules! deps {
    () => {
        Regex!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl core :: fmt :: Display for Regex { # [doc = " Shows the original regular expression."] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "{}" , self . as_str ()) } }
    };
}

impl_81!();