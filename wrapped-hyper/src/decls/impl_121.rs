macro_rules! deps {
    () => {
        TimedOut!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl StdError for TimedOut { }
    };
}

impl_121!()