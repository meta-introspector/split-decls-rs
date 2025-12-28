macro_rules! error_out_of_memory {
    () => {
        # [cfg (not (feature = "std"))] # [inline (always)] fn error_out_of_memory (msg : & 'static str) -> Error { Error :: OutOfMemory (msg) }
    };
}

error_out_of_memory!()