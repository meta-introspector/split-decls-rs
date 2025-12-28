macro_rules! deps {
    () => {
        RegisterRuleMap!();
        ReaderOffset!();
        UnwindContextStorage!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl < T , S > Eq for RegisterRuleMap < T , S > where T : ReaderOffset + Eq , S : UnwindContextStorage < T > , { }
    };
}

impl_228!()