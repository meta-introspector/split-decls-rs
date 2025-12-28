macro_rules! deps {
    () => {
        RuleType!();
        ErrorVariant!();
        Error!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < R : RuleType > core :: error :: Error for ErrorVariant < R > { }
    };
}

impl_5!()