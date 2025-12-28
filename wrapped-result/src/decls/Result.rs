macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " A specialized [`Result`] type that provides Windows error information."] pub type Result < T > = core :: result :: Result < T , Error > ;
    };
}

Result!();