macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " A typedef of the result returned by many methods."] pub type Result < T , E = Error > = result :: Result < T , E > ;
    };
}

Result!();