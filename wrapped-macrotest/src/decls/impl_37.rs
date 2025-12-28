macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl From < std :: io :: Error > for Error { fn from (e : std :: io :: Error) -> Self { Error :: Io (e) } }
    };
}

impl_37!()