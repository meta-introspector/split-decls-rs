macro_rules! deps {
    () => {
        RenderedExpandError!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl fmt :: Display for RenderedExpandError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . message) } }
    };
}

impl_26!()