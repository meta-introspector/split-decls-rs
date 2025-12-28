macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! FieldError {
    () => {
        deps!();
        # [doc = " An alias of [async_graphql::Error](struct.Error.html). Present for backward"] # [doc = " compatibility reasons."] pub type FieldError = Error ;
    };
}

FieldError!();