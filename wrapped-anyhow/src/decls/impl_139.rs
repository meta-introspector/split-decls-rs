macro_rules! deps {
    () => {
        StdError!();
        MessageError!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < M > StdError for MessageError < M > where M : Display + Debug + 'static { }
    };
}

impl_139!();