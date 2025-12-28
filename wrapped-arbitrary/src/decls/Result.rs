macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " A `Result` with the error type fixed as `arbitrary::Error`."] # [doc = ""] # [doc = " Either an `Ok(T)` or `Err(arbitrary::Error)`."] pub type Result < T , E = Error > = std :: result :: Result < T , E > ;
    };
}

Result!();