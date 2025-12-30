// Generated macro for set_scheme (function)
macro_rules! Depcrate_client_legacy_clientset_scheme {
() => {
// Module: crate::client::legacy::client
// Provides: {"set_scheme"}
// Dependencies: {}
fn set_scheme (uri : & mut Uri , scheme : Scheme) { debug_assert ! (uri . scheme () . is_none () , "set_scheme expects no existing scheme") ; let old = std :: mem :: take (uri) ; let mut parts : :: http :: uri :: Parts = old . into () ; parts . scheme = Some (scheme) ; parts . path_and_query = Some ("/" . parse () . expect ("slash is a valid path")) ; * uri = Uri :: from_parts (parts) . expect ("scheme is valid") ; }
};
}
