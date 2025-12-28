macro_rules! deps {
    () => {
        Logger!();
        Builder!();
        Formatter!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Builder { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if self . built { f . debug_struct ("Logger") . field ("built" , & true) . finish () } else { f . debug_struct ("Logger") . field ("filter" , & self . filter) . field ("writer" , & self . writer) . finish () } } }
    };
}

impl_4!()