macro_rules! deps {
    () => {
        Graph!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T : std :: fmt :: Debug > std :: fmt :: Debug for Graph < '_ , '_ , T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { std :: fmt :: Debug :: fmt (& self . map , f) } }
    };
}

impl_10!();