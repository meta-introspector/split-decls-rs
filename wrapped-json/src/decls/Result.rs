macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " Alias for a `Result` with the error type `serde_json::Error`."] pub type Result < T > = result :: Result < T , Error > ;
    };
}

Result!()