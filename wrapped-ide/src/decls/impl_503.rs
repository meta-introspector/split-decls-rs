macro_rules! deps {
    () => {
        TextPosition!();
    };
}

macro_rules! impl_503 {
    () => {
        deps!();
        impl std :: fmt :: Display for TextPosition { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "[{:?},{},{}]" , self . offset , self . line , self . col) } }
    };
}

impl_503!()