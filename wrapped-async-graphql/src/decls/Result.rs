macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " An alias for `Result<T, Error>`."] pub type Result < T , E = Error > = std :: result :: Result < T , E > ;
    };
}

Result!()