macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! eq_u64 {
    () => {
        deps!();
        fn eq_u64 (value : & Value , other : u64) -> bool { value . as_u64 () == Some (other) }
    };
}

eq_u64!();