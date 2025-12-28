macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Debug for File { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { write ! (f , r#"File("{:?}")"# , self . path . display ()) } }
    };
}

impl_10!();