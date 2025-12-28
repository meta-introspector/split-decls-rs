macro_rules! deps {
    () => {
        JsonUnexpected!();
        Error!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl de :: Error for Error { # [cold] fn custom < T : Display > (msg : T) -> Error { make_error (msg . to_string ()) } # [cold] fn invalid_type (unexp : de :: Unexpected , exp : & dyn de :: Expected) -> Self { Error :: custom (format_args ! ("invalid type: {}, expected {}" , JsonUnexpected (unexp) , exp ,)) } # [cold] fn invalid_value (unexp : de :: Unexpected , exp : & dyn de :: Expected) -> Self { Error :: custom (format_args ! ("invalid value: {}, expected {}" , JsonUnexpected (unexp) , exp ,)) } }
    };
}

impl_68!()