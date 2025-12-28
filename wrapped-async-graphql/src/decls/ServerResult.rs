macro_rules! deps {
    () => {
        ServerError!();
        Result!();
    };
}

macro_rules! ServerResult {
    () => {
        deps!();
        # [doc = " Alias for `Result<T, ServerError>`."] pub type ServerResult < T > = std :: result :: Result < T , ServerError > ;
    };
}

ServerResult!()