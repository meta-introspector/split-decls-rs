macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! eq_str {
    () => {
        deps!();
        fn eq_str (value : & Value , other : & str) -> bool { value . as_str () == Some (other) }
    };
}

eq_str!();