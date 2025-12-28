macro_rules! deps {
    () => {
        Attribute!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Attribute < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("Attribute") . field (& self . ctor () . parent () . name ()) . finish () } }
    };
}

impl_79!();