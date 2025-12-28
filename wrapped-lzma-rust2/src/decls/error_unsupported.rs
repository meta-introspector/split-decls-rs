macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! error_unsupported {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] # [inline (always)] fn error_unsupported (msg : & 'static str) -> Error { Error :: Unsupported (msg) }
    };
}

error_unsupported!()