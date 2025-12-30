// Generated macro for impl_133 (impl)
macro_rules! Depcrate_client_blocking_io_http_traitsimpl_133 {
() => {
// Module: crate::client::blocking_io::http::traits
// Provides: {"impl_133"}
// Dependencies: {}
impl crate :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: PostBody (err) => err . is_spurious () , # [cfg (any (feature = "http-client-reqwest" , feature = "http-client-curl"))] Error :: InitHttpClient { source } => { # [cfg (feature = "http-client-curl")] if let Some (err) = source . downcast_ref :: < crate :: client :: blocking_io :: http :: curl :: Error > () { return err . is_spurious () ; } # [cfg (feature = "http-client-reqwest")] if let Some (err) = source . downcast_ref :: < crate :: client :: blocking_io :: http :: reqwest :: remote :: Error > () { return err . is_spurious () ; } false } _ => false , } } }
};
}
