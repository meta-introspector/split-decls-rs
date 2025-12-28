macro_rules! deps {
    () => {
        LocalKey!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        impl < T : 'static > fmt :: Debug for LocalKey < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("LocalKey { .. }") } }
    };
}

impl_341!()