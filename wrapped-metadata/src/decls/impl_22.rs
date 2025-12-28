macro_rules! deps {
    () => {
        Blob!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Blob < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (f , "{:?}" , self . slice) } }
    };
}

impl_22!()