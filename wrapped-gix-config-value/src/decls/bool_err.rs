macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! bool_err {
    () => {
        deps!();
        fn bool_err (input : impl Into < BString >) -> Error { Error :: new ("Booleans need to be 'no', 'off', 'false', '' or 'yes', 'on', 'true' or any number" , input ,) }
    };
}

bool_err!()