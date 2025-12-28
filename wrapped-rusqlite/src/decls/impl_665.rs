macro_rules! deps {
    () => {
        SmallCString!();
        Result!();
    };
}

macro_rules! impl_665 {
    () => {
        deps!();
        impl std :: fmt :: Debug for SmallCString { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_tuple ("SmallCString") . field (& self . as_str ()) . finish () } }
    };
}

impl_665!();