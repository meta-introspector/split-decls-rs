macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        type Result < T , E = Error > = core :: result :: Result < T , E > ;
    };
}

Result!();