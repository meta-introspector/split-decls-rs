macro_rules! deps {
    () => {
        Error!();
        HRESULT!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl From < HRESULT > for Error { fn from (code : HRESULT) -> Self { Self { code : nonzero_hresult (code) , info : ErrorInfo :: from_thread () , } } }
    };
}

impl_66!()