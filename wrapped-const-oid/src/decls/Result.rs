macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " Result type"] pub type Result < T > = core :: result :: Result < T , Error > ;
    };
}

Result!()