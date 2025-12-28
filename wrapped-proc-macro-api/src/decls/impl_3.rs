macro_rules! deps {
    () => {
        SpanId!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl std :: fmt :: Debug for SpanId { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_3!();