macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " An alias for `Result<T, Error>`."] pub type Result < T > = std :: result :: Result < T , Error > ;
    };
}

Result!()