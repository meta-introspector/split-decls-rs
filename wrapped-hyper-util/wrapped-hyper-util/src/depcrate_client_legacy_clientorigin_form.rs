// Generated macro for origin_form (function)
macro_rules! Depcrate_client_legacy_clientorigin_form {
() => {
// Module: crate::client::legacy::client
// Provides: {"origin_form"}
// Dependencies: {}
fn origin_form (uri : & mut Uri) { let path = match uri . path_and_query () { Some (path) if path . as_str () != "/" => { let mut parts = :: http :: uri :: Parts :: default () ; parts . path_and_query = Some (path . clone ()) ; Uri :: from_parts (parts) . expect ("path is valid uri") } _none_or_just_slash => { debug_assert ! (Uri :: default () == "/") ; Uri :: default () } } ; * uri = path }
};
}
