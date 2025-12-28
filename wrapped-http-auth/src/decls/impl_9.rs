macro_rules! deps {
    () => {
        ParamsPrinter!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl std :: fmt :: Debug for ParamsPrinter < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_map () . entries (self . 0 . iter () . copied ()) . finish () } }
    };
}

impl_9!()