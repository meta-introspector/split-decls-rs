macro_rules! deps {
    () => {
        Error!();
        ErrorCode!();
    };
}

macro_rules! invalid_raw_value {
    () => {
        deps!();
        # [cfg (feature = "raw_value")] fn invalid_raw_value () -> Error { Error :: syntax (ErrorCode :: ExpectedSomeValue , 0 , 0) }
    };
}

invalid_raw_value!()