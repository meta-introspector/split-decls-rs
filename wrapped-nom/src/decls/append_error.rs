macro_rules! deps {
    () => {
        ErrorKind!();
        ParseError!();
    };
}

macro_rules! append_error {
    () => {
        deps!();
        # [doc = " Combines an existing error with a new one created from the input"] # [doc = " position and an [ErrorKind]. This is useful when backtracking"] # [doc = " through a parse tree, accumulating error context on the way"] pub fn append_error < I , E : ParseError < I > > (input : I , kind : ErrorKind , other : E) -> E { E :: append (input , kind , other) }
    };
}

append_error!();