macro_rules! deps {
    () => {
        ErrorKind!();
        ContextError!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < I > ContextError < I > for (I , ErrorKind) { }
    };
}

impl_23!();