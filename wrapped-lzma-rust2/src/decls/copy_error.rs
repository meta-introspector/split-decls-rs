macro_rules! copy_error {
    () => {
        # [cfg (not (feature = "std"))] # [inline (always)] fn copy_error (error : & Error) -> Error { * error }
    };
}

copy_error!()