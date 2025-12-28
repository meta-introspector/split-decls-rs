macro_rules! deps {
    () => {
        Outcome!();
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " The Result type used in credentials top-level functions to obtain a complete identity."] pub type Result = std :: result :: Result < Option < Outcome > , Error > ;
    };
}

Result!();