macro_rules! deps {
    () => {
        NestedClass!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl std :: fmt :: Debug for NestedClass < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_struct ("NestedClass") . field ("inner" , & self . inner ()) . field ("outer" , & self . outer ()) . finish () } }
    };
}

impl_114!();