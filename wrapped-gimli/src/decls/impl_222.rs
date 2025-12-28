macro_rules! deps {
    () => {
        UnwindContextStorage!();
        RegisterRuleMap!();
        ReaderOffset!();
        Result!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl < T , S > Debug for RegisterRuleMap < T , S > where T : ReaderOffset , S : UnwindContextStorage < T > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RegisterRuleMap") . field ("rules" , & self . rules) . finish () } }
    };
}

impl_222!();