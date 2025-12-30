// Generated macro for Header (enum)
macro_rules! Depcrate_hpack_headerHeader {
() => {
// Module: crate::hpack::header
// Provides: {"Header"}
// Dependencies: {}
# [doc = " HTTP/2 Header"] # [derive (Debug , Clone , Eq , PartialEq)] pub enum Header < T = HeaderName > { Field { name : T , value : HeaderValue } , Authority (BytesStr) , Method (Method) , Scheme (BytesStr) , Path (BytesStr) , Protocol (Protocol) , Status (StatusCode) , }
};
}
