macro_rules! deps {
    () => {
        Display!();
        RenderedExpandError!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl fmt :: Display for RenderedExpandError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . message) } }
    };
}

impl_232!()