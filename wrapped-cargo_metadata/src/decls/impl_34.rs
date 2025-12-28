macro_rules! deps {
    () => {
        CompilerMessage!();
        Result!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl fmt :: Display for CompilerMessage { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}" , self . message) } }
    };
}

impl_34!()