macro_rules! deps {
    () => {
        RuleType!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T : Copy + Debug + Eq + Hash + Ord > RuleType for T { }
    };
}

impl_15!()