macro_rules! deps {
    () => {
        ErrorKind!();
    };
}

macro_rules! FromExternalError {
    () => {
        deps!();
        # [doc = " This trait is required by the `map_res` combinator to integrate"] # [doc = " error types from external functions, like [std::str::FromStr]"] pub trait FromExternalError < I , E > { # [doc = " Creates a new error from an input position, an [ErrorKind] indicating the"] # [doc = " wrapping parser, and an external error"] fn from_external_error (input : I , kind : ErrorKind , e : E) -> Self ; }
    };
}

FromExternalError!();