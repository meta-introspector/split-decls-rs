macro_rules! error_invalid_input {
    () => {
        # [cfg (not (feature = "std"))] # [inline (always)] fn error_invalid_input (msg : & 'static str) -> Error { Error :: InvalidInput (msg) }
    };
}

error_invalid_input!()