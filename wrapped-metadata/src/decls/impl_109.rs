macro_rules! deps {
    () => {
        Module!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Module < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("Module") . field (& self . 0) . finish () } }
    };
}

impl_109!();