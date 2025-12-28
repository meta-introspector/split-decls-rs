macro_rules! deps {
    () => {
        ErrorKind!();
        Error!();
        ErrorFormatter!();
    };
}

macro_rules! impl_412 {
    () => {
        deps!();
        impl < F : ErrorFormatter > From < fmt :: Error > for Error < F > { fn from (e : fmt :: Error) -> Self { Error :: raw (ErrorKind :: Format , e) } }
    };
}

impl_412!()