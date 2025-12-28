macro_rules! deps {
    () => {
        Language!();
        SyntaxNode!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < L : Language > fmt :: Display for SyntaxNode < L > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . raw , f) } }
    };
}

impl_57!()