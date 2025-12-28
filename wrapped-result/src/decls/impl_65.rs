macro_rules! deps {
    () => {
        Error!();
        HRESULT!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl From < Error > for HRESULT { fn from (error : Error) -> Self { let code = error . code () ; error . info . into_thread () ; code } }
    };
}

impl_65!()