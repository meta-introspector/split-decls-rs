macro_rules! deps {
    () => {
        PatternError!();
        Pattern!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl fmt :: Display for PatternError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "Pattern syntax error near position {}: {}" , self . pos , self . msg) } }
    };
}

impl_18!()