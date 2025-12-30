// Generated macro for authority_form (function)
macro_rules! Depcrate_client_legacy_clientauthority_form {
() => {
// Module: crate::client::legacy::client
// Provides: {"authority_form"}
// Dependencies: {}
fn authority_form (uri : & mut Uri) { if let Some (path) = uri . path_and_query () { if path != "/" { warn ! ("HTTP/1.1 CONNECT request stripping path: {:?}" , path) ; } } * uri = match uri . authority () { Some (auth) => { let mut parts = :: http :: uri :: Parts :: default () ; parts . authority = Some (auth . clone ()) ; Uri :: from_parts (parts) . expect ("authority is valid") } None => { unreachable ! ("authority_form with relative uri") ; } } ; }
};
}
