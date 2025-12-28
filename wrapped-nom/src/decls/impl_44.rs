macro_rules! deps {
    () => {
        ErrorKind!();
        ParseError!();
        ErrorStr!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < I : Debug > ParseError < I > for ErrorStr { fn from_error_kind (input : I , kind : ErrorKind) -> Self { ErrorStr (format ! ("custom error message: ({:?}, {:?})" , input , kind)) } fn append (input : I , kind : ErrorKind , other : Self) -> Self { ErrorStr (format ! ("custom error message: ({:?}, {:?}) - {:?}" , input , kind , other)) } }
    };
}

impl_44!();