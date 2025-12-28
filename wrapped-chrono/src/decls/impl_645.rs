macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_645 {
    () => {
        deps!();
        impl From < SystemTimeError > for Error { fn from (error : SystemTimeError) -> Self { Error :: SystemTime (error) } }
    };
}

impl_645!();