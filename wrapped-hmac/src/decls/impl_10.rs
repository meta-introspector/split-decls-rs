macro_rules! deps {
    () => {
        HmacCore!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < D : EagerHash + AlgorithmName > AlgorithmName for HmacCore < D > { fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("Hmac<") ? ; < D as AlgorithmName > :: write_alg_name (f) ? ; f . write_str (">") } }
    };
}

impl_10!()