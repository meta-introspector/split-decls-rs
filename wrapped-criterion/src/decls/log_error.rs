macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! log_error {
    () => {
        deps!();
        pub (crate) fn log_error (e : & Error) { error ! ("error: {}" , e) ; }
    };
}

log_error!()