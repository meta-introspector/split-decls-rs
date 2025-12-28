macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " A `Result` alias where the `Err` case is `reqwest::Error`."] pub type Result < T > = std :: result :: Result < T , Error > ;
    };
}

Result!()