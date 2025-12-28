macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " The result of a write."] pub type Result < T > = result :: Result < T , Error > ;
    };
}

Result!()