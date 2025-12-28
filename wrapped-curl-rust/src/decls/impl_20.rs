macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl From < ffi :: NulError > for Error { fn from (_ : ffi :: NulError) -> Error { Error { code : curl_sys :: CURLE_CONV_FAILED , extra : None , } } }
    };
}

impl_20!();