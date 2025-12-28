macro_rules! deps {
    () => {
        RuleType!();
        ParseAttempts!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < R : RuleType > Default for ParseAttempts < R > { fn default () -> Self { Self :: new () } }
    };
}

impl_108!()