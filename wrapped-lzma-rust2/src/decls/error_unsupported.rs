macro_rules! error_unsupported {
    () => {
        # [cfg (not (feature = "std"))] # [inline (always)] fn error_unsupported (msg : & 'static str) -> Error { Error :: Unsupported (msg) }
    };
}

error_unsupported!()