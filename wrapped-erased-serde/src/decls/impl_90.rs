macro_rules! deps {
    () => {
        ErrorImpl!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl serde :: ser :: StdError for ErrorImpl { }
    };
}

impl_90!();