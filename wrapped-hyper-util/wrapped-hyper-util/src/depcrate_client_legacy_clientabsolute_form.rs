// Generated macro for absolute_form (function)
macro_rules! Depcrate_client_legacy_clientabsolute_form {
() => {
// Module: crate::client::legacy::client
// Provides: {"absolute_form"}
// Dependencies: {}
fn absolute_form (uri : & mut Uri) { debug_assert ! (uri . scheme () . is_some () , "absolute_form needs a scheme") ; debug_assert ! (uri . authority () . is_some () , "absolute_form needs an authority") ; }
};
}
