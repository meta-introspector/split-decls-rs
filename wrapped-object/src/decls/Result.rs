macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " The result type used within the build module."] pub type Result < T > = result :: Result < T , Error > ;
    };
}

Result!();