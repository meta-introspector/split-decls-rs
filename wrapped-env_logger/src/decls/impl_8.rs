macro_rules! deps {
    () => {
        Formatter!();
        Logger!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Logger { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Logger") . field ("filter" , & self . filter) . finish () } }
    };
}

impl_8!()