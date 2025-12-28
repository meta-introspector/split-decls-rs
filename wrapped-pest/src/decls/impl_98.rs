macro_rules! deps {
    () => {
        ParseAttempt!();
        RulesCallStack!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < R > RulesCallStack < R > { fn new (deepest : ParseAttempt < R >) -> RulesCallStack < R > { RulesCallStack { deepest , parent : None , } } }
    };
}

impl_98!();