macro_rules! deps {
    () => {
        InputStructField!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl fmt :: Display for InputStructField { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . name) } }
    };
}

impl_23!();