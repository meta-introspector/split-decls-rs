macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_644 {
    () => {
        deps!();
        impl From < ParseIntError > for Error { fn from (error : ParseIntError) -> Self { Error :: ParseInt (error) } }
    };
}

impl_644!();