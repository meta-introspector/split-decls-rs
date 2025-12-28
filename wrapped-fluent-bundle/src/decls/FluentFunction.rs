macro_rules! deps {
    () => {
        FluentValue!();
        FluentArgs!();
    };
}

macro_rules! FluentFunction {
    () => {
        deps!();
        pub type FluentFunction = Box < dyn for < 'a > Fn (& [FluentValue < 'a >] , & FluentArgs) -> FluentValue < 'a > + Send + Sync > ;
    };
}

FluentFunction!()