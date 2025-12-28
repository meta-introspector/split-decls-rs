macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! int_err {
    () => {
        deps!();
        fn int_err (input : impl Into < BString >) -> Error { Error :: new ("Integers needs to be positive or negative numbers which may have a suffix like 1k, 42, or 50G" , input ,) }
    };
}

int_err!()