macro_rules! deps {
    () => {
        InterfaceImpl!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl std :: fmt :: Debug for InterfaceImpl < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("InterfaceImpl") . field (& self . 0) . finish () } }
    };
}

impl_97!();