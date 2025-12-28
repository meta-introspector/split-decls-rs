macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl From < getrandom :: Error > for Error { fn from (_ : getrandom :: Error) -> Self { Error :: Csprng } }
    };
}

impl_10!();