macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Field < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("Field") . field (& self . name ()) . finish () } }
    };
}

impl_88!()