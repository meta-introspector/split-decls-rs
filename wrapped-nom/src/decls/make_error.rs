macro_rules! deps {
    () => {
        ErrorKind!();
        ParseError!();
    };
}

macro_rules! make_error {
    () => {
        deps!();
        # [doc = " Creates an error from the input position and an [ErrorKind]"] pub fn make_error < I , E : ParseError < I > > (input : I , kind : ErrorKind) -> E { E :: from_error_kind (input , kind) }
    };
}

make_error!()