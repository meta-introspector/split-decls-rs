macro_rules! deps {
    () => {
        HeaderValue!();
        ToStrError!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        # [cfg (feature = "http")] impl HeaderValue for http :: HeaderValue { fn to_str (& self) -> Result < & str , ToStrError > { self . to_str () . map_err (| _ | ToStrError { _priv : () }) } }
    };
}

impl_70!();