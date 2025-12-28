macro_rules! deps {
    () => {
        Tree!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Tree < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "Tree({})" , self . id) } }
    };
}

impl_230!();