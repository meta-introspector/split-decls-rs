macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Entry < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Entry") . field ("path_buf" , & self . relative_path ()) . field ("mode" , & self . mode) . field ("id" , & self . id) . field ("remaining" , & self . remaining) . finish () } }
    };
}

impl_5!();