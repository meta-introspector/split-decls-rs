macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! io {
    () => {
        deps!();
        # [doc = " Error types for utility hash functions"] pub mod io ;
    };
}

io!();