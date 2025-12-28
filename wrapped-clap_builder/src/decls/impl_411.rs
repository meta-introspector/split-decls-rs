macro_rules! deps {
    () => {
        ErrorFormatter!();
        Error!();
        ErrorKind!();
    };
}

macro_rules! impl_411 {
    () => {
        deps!();
        impl < F : ErrorFormatter > From < io :: Error > for Error < F > { fn from (e : io :: Error) -> Self { Error :: raw (ErrorKind :: Io , e) } }
    };
}

impl_411!()