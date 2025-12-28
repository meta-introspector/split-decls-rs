macro_rules! deps {
    () => {
        PotentialUtf8!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl fmt :: Debug for PotentialUtf8 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . try_as_str () { Ok (s) => fmt :: Debug :: fmt (s , f) , Err (_) => fmt :: Debug :: fmt (& self . 0 , f) , } } }
    };
}

impl_22!()