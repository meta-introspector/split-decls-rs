macro_rules! deps {
    () => {
        AttrPath!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl fmt :: Display for AttrPath { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , join_path_idents (& self . segments)) } }
    };
}

impl_159!()