macro_rules! deps {
    () => {
        ToStrError!();
        HeaderValue!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        # [cfg (feature = "http10")] impl HeaderValue for http10 :: HeaderValue { fn to_str (& self) -> Result < & str , ToStrError > { self . to_str () . map_err (| _ | ToStrError { _priv : () }) } }
    };
}

impl_71!()