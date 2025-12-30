// Generated macro for impl_285 (impl)
macro_rules! Depcrate_common_authorizationimpl_285 {
() => {
// Module: crate::common::authorization
// Provides: {"impl_285"}
// Dependencies: {}
impl Credentials for Basic { const SCHEME : & 'static str = "Basic" ; fn decode (value : & HeaderValue) -> Option < Self > { debug_assert ! (value . as_bytes () [.. Self :: SCHEME . len ()] . eq_ignore_ascii_case (Self :: SCHEME . as_bytes ()) , "HeaderValue to decode should start with \"Basic ..\", received = {:?}" , value ,) ; let bytes = value . as_bytes () . get ("Basic " . len () ..) ? ; let non_space_pos = bytes . iter () . position (| b | * b != b' ') ? ; let bytes = & bytes [non_space_pos ..] ; let bytes = ENGINE . decode (bytes) . ok () ? ; let decoded = String :: from_utf8 (bytes) . ok () ? ; let colon_pos = decoded . find (':') ? ; Some (Basic { decoded , colon_pos }) } fn encode (& self) -> HeaderValue { let mut encoded = String :: from ("Basic ") ; ENGINE . encode_string (& self . decoded , & mut encoded) ; let bytes = Bytes :: from (encoded) ; HeaderValue :: from_maybe_shared (bytes) . expect ("base64 encoding is always a valid HeaderValue") } }
};
}
