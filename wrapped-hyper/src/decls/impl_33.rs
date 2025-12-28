macro_rules! deps {
    () => {
        DecodedLength!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        # [cfg (any (feature = "http1" , feature = "http2"))] impl From < Option < u64 > > for DecodedLength { fn from (len : Option < u64 >) -> Self { len . and_then (| len | { Self :: checked_new (len) . ok () }) . unwrap_or (DecodedLength :: CHUNKED) } }
    };
}

impl_33!();