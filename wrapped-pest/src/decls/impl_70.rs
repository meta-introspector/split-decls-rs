macro_rules! deps {
    () => {
        RuleType!();
        Tokens!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < R : RuleType > fmt :: Debug for Tokens < '_ , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
    };
}

impl_70!()