macro_rules! deps {
    () => {
        HmacCore!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < D : EagerHash > fmt :: Debug for HmacCore < D > where D :: Core : AlgorithmName , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("HmacCore { ... }") } }
    };
}

impl_11!()