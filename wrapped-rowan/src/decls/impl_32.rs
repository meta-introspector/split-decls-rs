macro_rules! deps {
    () => {
        SyntaxToken!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl fmt :: Display for SyntaxToken { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self . text () , f) } }
    };
}

impl_32!();