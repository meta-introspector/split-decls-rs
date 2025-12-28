macro_rules! deps {
    () => {
        Error!();
        ErrorCode!();
    };
}

macro_rules! key_must_be_a_string {
    () => {
        deps!();
        fn key_must_be_a_string () -> Error { Error :: syntax (ErrorCode :: KeyMustBeAString , 0 , 0) }
    };
}

key_must_be_a_string!();