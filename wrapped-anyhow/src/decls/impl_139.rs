macro_rules! deps {
    () => {
        MessageError!();
        StdError!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < M > StdError for MessageError < M > where M : Display + Debug + 'static { }
    };
}

impl_139!()