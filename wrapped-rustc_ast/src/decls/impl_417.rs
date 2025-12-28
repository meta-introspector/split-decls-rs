macro_rules! deps {
    () => {
        NonterminalKind!();
    };
}

macro_rules! impl_417 {
    () => {
        deps!();
        impl fmt :: Display for NonterminalKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . symbol ()) } }
    };
}

impl_417!();