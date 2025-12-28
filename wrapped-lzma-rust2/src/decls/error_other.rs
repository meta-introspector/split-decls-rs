macro_rules! error_other {
    () => {
        # [cfg (not (feature = "std"))] # [inline (always)] fn error_other (msg : & 'static str) -> Error { Error :: Other (msg) }
    };
}

error_other!()