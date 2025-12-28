macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl From < core :: num :: TryFromIntError > for Error { fn from (_ : core :: num :: TryFromIntError) -> Self { Error :: LossyConversion } }
    };
}

impl_11!();