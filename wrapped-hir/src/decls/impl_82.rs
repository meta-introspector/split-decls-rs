macro_rules! deps {
    () => {
        Semantics!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < DB : ? Sized > fmt :: Debug for Semantics < '_ , DB > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Semantics {{ ... }}") } }
    };
}

impl_82!();