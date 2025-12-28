macro_rules! deps {
    () => {
        WIN32_ERROR!();
        Error!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl From < alloc :: string :: FromUtf16Error > for Error { fn from (_ : alloc :: string :: FromUtf16Error) -> Self { WIN32_ERROR (ERROR_NO_UNICODE_TRANSLATION) . into () } }
    };
}

impl_69!()