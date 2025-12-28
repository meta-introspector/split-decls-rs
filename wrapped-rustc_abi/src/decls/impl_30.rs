macro_rules! deps {
    () => {
        ExternAbi!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl fmt :: Display for ExternAbi { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "\"{}\"" , self . as_str ()) } }
    };
}

impl_30!()