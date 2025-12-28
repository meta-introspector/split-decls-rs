macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl fmt :: Display for Position { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}:{}:{}" , self . file , self . line , self . column) } }
    };
}

impl_7!();