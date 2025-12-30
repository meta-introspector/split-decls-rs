// Generated macro for impl_11 (impl)
macro_rules! Depcrate_helperimpl_11 {
() => {
// Module: crate::helper
// Provides: {"impl_11"}
// Dependencies: {}
# [doc = " Initialization"] impl Action { # [doc = " Create a `Get` action with context containing the given URL."] # [doc = " Note that this creates an `Action` suitable for the credential helper cascade only."] pub fn get_for_url (url : impl Into < BString >) -> Action { Action :: Get (Context { url : Some (url . into ()) , .. Default :: default () }) } }
};
}
