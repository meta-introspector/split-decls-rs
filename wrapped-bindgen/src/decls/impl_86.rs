macro_rules! impl_86 {
    () => {
        impl std :: fmt :: Debug for InterfaceImpl { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("InterfaceImpl") . field (& self . 0) . finish () } }
    };
}

impl_86!();