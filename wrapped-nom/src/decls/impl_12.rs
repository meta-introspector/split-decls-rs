macro_rules! deps {
    () => {
        Error!();
        ContextError!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < I > ContextError < I > for Error < I > { }
    };
}

impl_12!();