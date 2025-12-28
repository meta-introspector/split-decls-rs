macro_rules! error_eof {
    () => {
        # [cfg (not (feature = "std"))] # [inline (always)] fn error_eof () -> Error { Error :: Eof }
    };
}

error_eof!()