macro_rules! deps {
    () => {
        ReaderOffset!();
        UnwindContextStorage!();
        RegisterRuleMap!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl < T , S > Clone for RegisterRuleMap < T , S > where T : ReaderOffset , S : UnwindContextStorage < T > , { fn clone (& self) -> Self { Self { rules : self . rules . clone () , } } }
    };
}

impl_223!();