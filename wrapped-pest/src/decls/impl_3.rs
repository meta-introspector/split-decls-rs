macro_rules! deps {
    () => {
        RuleType!();
        Error!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < R : RuleType > core :: error :: Error for Error < R > { }
    };
}

impl_3!()