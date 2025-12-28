macro_rules! deps {
    () => {
        TimedOut!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl StdError for TimedOut { }
    };
}

impl_29!();