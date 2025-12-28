macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! error_invalid_data {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] # [inline (always)] fn error_invalid_data (msg : & 'static str) -> Error { Error :: InvalidData (msg) }
    };
}

error_invalid_data!();