macro_rules! deps {
    () => {
        Var!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl Display for Var { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "${}" , self . 0) } }
    };
}

impl_63!()