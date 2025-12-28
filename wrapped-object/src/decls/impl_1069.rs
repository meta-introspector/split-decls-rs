macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_1069 {
    () => {
        deps!();
        impl From < write :: Error > for Error { fn from (error : write :: Error) -> Error { Error (error . 0) } }
    };
}

impl_1069!()