macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! eq_f64 {
    () => {
        deps!();
        fn eq_f64 (value : & Value , other : f64) -> bool { value . as_f64 () == Some (other) }
    };
}

eq_f64!();