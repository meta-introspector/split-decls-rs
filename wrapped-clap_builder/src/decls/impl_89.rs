macro_rules! deps {
    () => {
        Command!();
        Result!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl fmt :: Display for Command { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . name) } }
    };
}

impl_89!();