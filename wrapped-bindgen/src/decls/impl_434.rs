macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! impl_434 {
    () => {
        deps!();
        impl std :: fmt :: Debug for File { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { std :: write ! (f , "{:?}" , self . bytes . as_ptr ()) } }
    };
}

impl_434!();