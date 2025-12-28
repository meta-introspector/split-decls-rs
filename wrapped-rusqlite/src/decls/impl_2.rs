macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl From < str :: Utf8Error > for Error { # [cold] fn from (err : str :: Utf8Error) -> Self { Self :: Utf8Error (err) } }
    };
}

impl_2!()