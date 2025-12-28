macro_rules! deps {
    () => {
        ExternAbi!();
    };
}

macro_rules! macro_25 {
    () => {
        deps!();
        # [cfg (feature = "nightly")] rustc_error_messages :: into_diag_arg_using_display ! (ExternAbi) ;
    };
}

macro_25!();