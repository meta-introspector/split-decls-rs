macro_rules! impl_101 {
    () => {
        impl std :: fmt :: Debug for NestedClass { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_struct ("NestedClass") . field ("inner" , & self . inner ()) . field ("outer" , & self . outer ()) . finish () } }
    };
}

impl_101!();