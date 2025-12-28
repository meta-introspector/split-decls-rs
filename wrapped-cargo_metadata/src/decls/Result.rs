macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " Custom result type for `cargo_metadata::Error`"] pub type Result < T , E = Error > = :: std :: result :: Result < T , E > ;
    };
}

Result!();