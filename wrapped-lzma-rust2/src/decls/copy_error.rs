macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! copy_error {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] # [inline (always)] fn copy_error (error : & Error) -> Error { * error }
    };
}

copy_error!()