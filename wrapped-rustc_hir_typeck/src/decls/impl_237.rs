macro_rules! deps {
    () => {
        BreakContextKind!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl fmt :: Display for BreakContextKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { BreakContextKind :: Break => "break" , BreakContextKind :: Continue => "continue" , } . fmt (f) } }
    };
}

impl_237!();