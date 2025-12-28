macro_rules! deps {
    () => {
        HmacResetCore!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < D : EagerHash > fmt :: Debug for HmacResetCore < D > where D :: Core : AlgorithmName , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("HmacResetCore { ... }") } }
    };
}

impl_24!()