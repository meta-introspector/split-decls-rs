macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_646 {
    () => {
        deps!();
        impl From < Utf8Error > for Error { fn from (error : Utf8Error) -> Self { Error :: Utf8 (error) } }
    };
}

impl_646!();