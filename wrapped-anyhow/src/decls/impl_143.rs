macro_rules! deps {
    () => {
        DisplayError!();
        StdError!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < M > StdError for DisplayError < M > where M : Display + 'static { }
    };
}

impl_143!()