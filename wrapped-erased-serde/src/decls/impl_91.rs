macro_rules! deps {
    () => {
        ErrorImpl!();
        Error!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl serde :: ser :: Error for ErrorImpl { fn custom < T : Display > (msg : T) -> Self { ErrorImpl :: Custom (Box :: new (msg . to_string ())) } }
    };
}

impl_91!();