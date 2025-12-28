macro_rules! deps {
    () => {
        Commit!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Commit < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "Commit({})" , self . id) } }
    };
}

impl_160!()