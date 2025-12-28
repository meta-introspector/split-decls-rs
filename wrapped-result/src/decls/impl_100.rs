macro_rules! deps {
    () => {
        WIN32_ERROR!();
        HRESULT!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl From < WIN32_ERROR > for HRESULT { fn from (value : WIN32_ERROR) -> Self { value . to_hresult () } }
    };
}

impl_100!()