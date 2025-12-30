// Generated macro for impl_416 (impl)
macro_rules! Depcrate_common_hostimpl_416 {
() => {
// Module: crate::common::host
// Provides: {"impl_416"}
// Dependencies: {}
impl Header for Host { fn name () -> & 'static HeaderName { & :: http :: header :: HOST } fn decode < 'i , I : Iterator < Item = & 'i HeaderValue > > (values : & mut I) -> Result < Self , Error > { values . next () . cloned () . and_then (| val | Authority :: try_from (val . as_bytes ()) . ok ()) . map (Host) . ok_or_else (Error :: invalid) } fn encode < E : Extend < HeaderValue > > (& self , values : & mut E) { let bytes = self . 0 . as_str () . as_bytes () ; let val = HeaderValue :: from_bytes (bytes) . expect ("Authority is a valid HeaderValue") ; values . extend (:: std :: iter :: once (val)) ; } }
};
}
