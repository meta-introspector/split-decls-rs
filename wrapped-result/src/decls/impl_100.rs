macro_rules! deps {
    () => {
        HRESULT!();
        WIN32_ERROR!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl From < WIN32_ERROR > for HRESULT { fn from (value : WIN32_ERROR) -> Self { value . to_hresult () } }
    };
}

impl_100!();