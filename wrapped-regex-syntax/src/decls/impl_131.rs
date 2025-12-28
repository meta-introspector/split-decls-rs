macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl From < ast :: Error > for Error { fn from (err : ast :: Error) -> Error { Error :: Parse (err) } }
    };
}

impl_131!()