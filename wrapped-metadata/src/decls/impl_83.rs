macro_rules! deps {
    () => {
        ClassLayout!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl std :: fmt :: Debug for ClassLayout < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("ClassLayout") . field (& self . packing_size ()) . finish () } }
    };
}

impl_83!();