macro_rules! deps {
    () => {
        ErrorKind!();
        Input!();
        Char!();
    };
}

macro_rules! ParseError {
    () => {
        deps!();
        # [doc = " This trait must be implemented by the error type of a nom parser."] # [doc = ""] # [doc = " There are already implementations of it for `(Input, ErrorKind)`"] # [doc = " and `Error<Input>`."] # [doc = ""] # [doc = " It provides methods to create an error from some combinators,"] # [doc = " and combine existing errors in combinators like `alt`."] pub trait ParseError < I > : Sized { # [doc = " Creates an error from the input position and an [ErrorKind]"] fn from_error_kind (input : I , kind : ErrorKind) -> Self ; # [doc = " Combines an existing error with a new one created from the input"] # [doc = " position and an [ErrorKind]. This is useful when backtracking"] # [doc = " through a parse tree, accumulating error context on the way"] fn append (input : I , kind : ErrorKind , other : Self) -> Self ; # [doc = " Creates an error from an input position and an expected character"] fn from_char (input : I , _ : char) -> Self { Self :: from_error_kind (input , ErrorKind :: Char) } # [doc = " Combines two existing errors. This function is used to compare errors"] # [doc = " generated in various branches of `alt`."] fn or (self , other : Self) -> Self { other } }
    };
}

ParseError!();