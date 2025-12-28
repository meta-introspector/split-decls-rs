macro_rules! deps {
    () => {
        Error!();
        ErrorCode!();
    };
}

macro_rules! invalid_number {
    () => {
        deps!();
        # [cfg (feature = "arbitrary_precision")] fn invalid_number () -> Error { Error :: syntax (ErrorCode :: InvalidNumber , 0 , 0) }
    };
}

invalid_number!();