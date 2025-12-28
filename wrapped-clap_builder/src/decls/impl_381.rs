macro_rules! deps {
    () => {
        Result!();
        ContextKind!();
    };
}

macro_rules! impl_381 {
    () => {
        deps!();
        impl std :: fmt :: Display for ContextKind { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . as_str () . unwrap_or_default () . fmt (f) } }
    };
}

impl_381!()