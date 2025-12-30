// Generated macro for minimize_rpaths (function)
macro_rules! Depcrate_back_rpathminimize_rpaths {
() => {
// Module: crate::back::rpath
// Provides: {"minimize_rpaths"}
// Dependencies: {}
fn minimize_rpaths (rpaths : & [OsString]) -> Vec < OsString > { let mut set = FxHashSet :: default () ; let mut minimized = Vec :: new () ; for rpath in rpaths { if set . insert (rpath) { minimized . push (rpath . clone ()) ; } } minimized }
};
}
