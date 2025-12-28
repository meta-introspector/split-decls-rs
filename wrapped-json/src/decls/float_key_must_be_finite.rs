macro_rules! deps {
    () => {
        Error!();
        ErrorCode!();
    };
}

macro_rules! float_key_must_be_finite {
    () => {
        deps!();
        fn float_key_must_be_finite () -> Error { Error :: syntax (ErrorCode :: FloatKeyMustBeFinite , 0 , 0) }
    };
}

float_key_must_be_finite!()