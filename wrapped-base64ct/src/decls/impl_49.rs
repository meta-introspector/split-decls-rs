macro_rules! deps {
    () => {
        InvalidEncodingError!();
        Error!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl core :: error :: Error for InvalidEncodingError { }
    };
}

impl_49!()