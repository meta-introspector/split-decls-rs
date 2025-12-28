macro_rules! deps {
    () => {
        RuleType!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < T : Copy + Debug + Eq + Hash + Ord > RuleType for T { }
    };
}

impl_180!()