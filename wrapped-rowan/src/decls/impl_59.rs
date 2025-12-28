macro_rules! deps {
    () => {
        Language!();
        SyntaxToken!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < L : Language > fmt :: Display for SyntaxToken < L > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . raw , f) } }
    };
}

impl_59!();