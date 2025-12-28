macro_rules! deps {
    () => {
        Target!();
    };
}

macro_rules! macro_473 {
    () => {
        deps!();
        rustc_error_messages :: into_diag_arg_using_display ! (Target) ;
    };
}

macro_473!();