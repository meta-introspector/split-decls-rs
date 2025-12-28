macro_rules! deps {
    () => {
        RuleType!();
        PrattParser!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl < R : RuleType > Default for PrattParser < R > { fn default () -> Self { Self :: new () } }
    };
}

impl_134!()