// Generated macro for add_test_support_redactions (function)
macro_rules! Depcrate_compareadd_test_support_redactions {
() => {
// Module: crate::compare
// Provides: {"add_test_support_redactions"}
// Dependencies: {}
fn add_test_support_redactions (subs : & mut snapbox :: Redactions) { let root = paths :: root () ; let root_url = url :: Url :: from_file_path (& root) . unwrap () . to_string () ; subs . insert ("[ROOT]" , root) . unwrap () ; subs . insert ("[ROOTURL]" , root_url) . unwrap () ; subs . insert ("[HOST_TARGET]" , rustc_host ()) . unwrap () ; if let Some (alt_target) = try_alternate () { subs . insert ("[ALT_TARGET]" , alt_target) . unwrap () ; } }
};
}
