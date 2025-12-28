macro_rules! deps {
    () => {
        FluentError!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Error for FluentError { }
    };
}

impl_29!()