macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " Result type alias where the error is `erased_serde::Error`."] pub type Result < T > = core :: result :: Result < T , Error > ;
    };
}

Result!()