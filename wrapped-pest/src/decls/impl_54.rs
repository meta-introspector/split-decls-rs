macro_rules! deps {
    () => {
        RuleType!();
        Pairs!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < R : RuleType > fmt :: Debug for Pairs < '_ , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
    };
}

impl_54!()