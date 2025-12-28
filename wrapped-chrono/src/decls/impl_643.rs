macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_643 {
    () => {
        deps!();
        impl From < io :: Error > for Error { fn from (error : io :: Error) -> Self { Error :: Io (error) } }
    };
}

impl_643!();