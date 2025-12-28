macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! eq_i64 {
    () => {
        deps!();
        fn eq_i64 (value : & Value , other : i64) -> bool { value . as_i64 () == Some (other) }
    };
}

eq_i64!()