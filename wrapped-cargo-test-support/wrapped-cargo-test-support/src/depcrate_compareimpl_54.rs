// Generated macro for impl_54 (impl)
macro_rules! Depcrate_compareimpl_54 {
() => {
// Module: crate::compare
// Provides: {"impl_54"}
// Dependencies: {}
impl InMemoryDir { pub fn paths (& self) -> impl Iterator < Item = & Path > { self . files . iter () . map (| (p , _) | p . as_path ()) } # [track_caller] pub fn assert_contains (& self , expected : & Self) { use std :: fmt :: Write as _ ; let assert = assert_e2e () ; let mut errs = String :: new () ; for (path , expected_data) in & expected . files { let actual_data = self . files . iter () . find_map (| (p , d) | (path == p) . then (| | d . clone ())) . unwrap_or_else (| | Data :: new ()) ; if let Err (err) = assert . try_eq (Some (& path . display ()) , actual_data , expected_data . clone ()) { let _ = write ! (& mut errs , "{err}") ; } } if ! errs . is_empty () { panic ! ("{errs}") } } }
};
}
