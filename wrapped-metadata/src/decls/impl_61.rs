macro_rules! deps {
    () => {
        Row!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Row < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_struct ("Row") . field ("file" , & self . file) . field ("pos" , & self . pos) . finish () } }
    };
}

impl_61!();