macro_rules! deps {
    () => {
        NameValidationError!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        type Result < T > = std :: result :: Result < T , NameValidationError > ;
    };
}

Result!();