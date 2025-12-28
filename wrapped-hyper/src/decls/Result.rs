macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " Result type often returned from methods that can have hyper `Error`s."] pub type Result < T > = std :: result :: Result < T , Error > ;
    };
}

Result!();