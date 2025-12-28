macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! impl_330 {
    () => {
        deps!();
        impl std :: fmt :: Debug for crate :: Repository { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Repository") . field ("kind" , & self . kind ()) . field ("git_dir" , & self . git_dir ()) . field ("workdir" , & self . workdir ()) . finish () } }
    };
}

impl_330!()