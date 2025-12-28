macro_rules! deps {
    () => {
        HmacResetCore!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < D : EagerHash + AlgorithmName > AlgorithmName for HmacResetCore < D > { fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("Hmac<") ? ; < D as AlgorithmName > :: write_alg_name (f) ? ; f . write_str (">") } }
    };
}

impl_23!();