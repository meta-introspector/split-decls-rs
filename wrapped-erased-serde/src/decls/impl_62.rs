macro_rules! deps {
    () => {
        Error!();
        ErrorImpl!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl serde :: ser :: Error for Error { fn custom < T : Display > (msg : T) -> Self { let imp = Box :: new (ErrorImpl :: Custom (msg . to_string ())) ; Error { imp } } }
    };
}

impl_62!()