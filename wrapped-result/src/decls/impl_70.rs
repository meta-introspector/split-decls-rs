macro_rules! deps {
    () => {
        WIN32_ERROR!();
        Error!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl From < alloc :: string :: FromUtf8Error > for Error { fn from (_ : alloc :: string :: FromUtf8Error) -> Self { WIN32_ERROR (ERROR_NO_UNICODE_TRANSLATION) . into () } }
    };
}

impl_70!()