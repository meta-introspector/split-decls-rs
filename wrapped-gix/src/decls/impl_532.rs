macro_rules! deps {
    () => {
        Cache!();
    };
}

macro_rules! impl_532 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Cache { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Cache") . finish_non_exhaustive () } }
    };
}

impl_532!();