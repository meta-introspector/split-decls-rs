macro_rules! deps {
    () => {
        WIN32_ERROR!();
        Error!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl From < core :: num :: TryFromIntError > for Error { fn from (_ : core :: num :: TryFromIntError) -> Self { WIN32_ERROR (ERROR_INVALID_DATA) . into () } }
    };
}

impl_71!()