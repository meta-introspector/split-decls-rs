macro_rules! deps {
    () => {
        InputValueError!();
        Result!();
    };
}

macro_rules! InputValueResult {
    () => {
        deps!();
        # [doc = " An error parsing a value of type `T`."] pub type InputValueResult < T > = Result < T , InputValueError < T > > ;
    };
}

InputValueResult!();