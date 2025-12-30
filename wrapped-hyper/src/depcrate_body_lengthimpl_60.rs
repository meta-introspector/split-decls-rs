// Generated macro for impl_60 (impl)
macro_rules! Depcrate_body_lengthimpl_60 {
() => {
// Module: crate::body::length
// Provides: {"impl_60"}
// Dependencies: {}
# [cfg (any (feature = "http1" , feature = "http2"))] impl From < Option < u64 > > for DecodedLength { fn from (len : Option < u64 >) -> Self { len . and_then (| len | { Self :: checked_new (len) . ok () }) . unwrap_or (DecodedLength :: CHUNKED) } }
};
}
