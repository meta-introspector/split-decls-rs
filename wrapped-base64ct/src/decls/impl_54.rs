macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl From < core :: str :: Utf8Error > for Error { # [inline] fn from (_ : core :: str :: Utf8Error) -> Error { Error :: InvalidEncoding } }
    };
}

impl_54!()