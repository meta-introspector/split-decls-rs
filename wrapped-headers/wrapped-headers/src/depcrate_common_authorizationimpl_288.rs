// Generated macro for impl_288 (impl)
macro_rules! Depcrate_common_authorizationimpl_288 {
() => {
// Module: crate::common::authorization
// Provides: {"impl_288"}
// Dependencies: {}
impl Credentials for Bearer { const SCHEME : & 'static str = "Bearer" ; fn decode (value : & HeaderValue) -> Option < Self > { debug_assert ! (value . as_bytes () [.. Self :: SCHEME . len ()] . eq_ignore_ascii_case (Self :: SCHEME . as_bytes ()) , "HeaderValue to decode should start with \"Bearer ..\", received = {:?}" , value ,) ; HeaderValueString :: from_val (value) . ok () . map (Bearer) } fn encode (& self) -> HeaderValue { (& self . 0) . into () } }
};
}
