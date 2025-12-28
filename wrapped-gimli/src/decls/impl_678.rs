macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_678 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl From < io :: Error > for Error { fn from (_ : io :: Error) -> Self { Error :: Io } }
    };
}

impl_678!()