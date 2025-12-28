macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! error_invalid_input {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] # [inline (always)] fn error_invalid_input (msg : & 'static str) -> Error { Error :: InvalidInput (msg) }
    };
}

error_invalid_input!()