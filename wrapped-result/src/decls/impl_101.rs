macro_rules! deps {
    () => {
        Error!();
        WIN32_ERROR!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl From < WIN32_ERROR > for Error { fn from (value : WIN32_ERROR) -> Self { value . to_hresult () . into () } }
    };
}

impl_101!()