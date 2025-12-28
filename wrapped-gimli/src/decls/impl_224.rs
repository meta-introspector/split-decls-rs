macro_rules! deps {
    () => {
        ReaderOffset!();
        UnwindContextStorage!();
        RegisterRuleMap!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl < T , S > Default for RegisterRuleMap < T , S > where T : ReaderOffset , S : UnwindContextStorage < T > , { fn default () -> Self { RegisterRuleMap { rules : Default :: default () , } } }
    };
}

impl_224!();