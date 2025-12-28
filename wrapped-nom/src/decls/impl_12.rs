macro_rules! deps {
    () => {
        ContextError!();
        Error!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < I > ContextError < I > for Error < I > { }
    };
}

impl_12!()