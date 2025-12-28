macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! error_eof {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] # [inline (always)] fn error_eof () -> Error { Error :: Eof }
    };
}

error_eof!()