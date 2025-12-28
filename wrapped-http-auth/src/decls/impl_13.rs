macro_rules! deps {
    () => {
        ToStrError!();
        HeaderValue!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        # [cfg (feature = "http")] impl HeaderValue for http :: HeaderValue { fn to_str (& self) -> Result < & str , ToStrError > { self . to_str () . map_err (| _ | ToStrError { _priv : () }) } }
    };
}

impl_13!()