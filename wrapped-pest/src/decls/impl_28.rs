macro_rules! deps {
    () => {
        FlatPairs!();
        RuleType!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < R : RuleType > fmt :: Debug for FlatPairs < '_ , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FlatPairs") . field ("pairs" , & self . clone () . collect :: < Vec < _ > > ()) . finish () } }
    };
}

impl_28!();