macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! last_error {
    () => {
        deps!();
        pub fn last_error (code : libc :: c_int) -> Error { Error :: last_error (code) }
    };
}

last_error!()