macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! eq_bool {
    () => {
        deps!();
        fn eq_bool (value : & Value , other : bool) -> bool { value . as_bool () == Some (other) }
    };
}

eq_bool!();