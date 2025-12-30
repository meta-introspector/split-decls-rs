// Generated macro for ensure_removed (function)
macro_rules! Depcrate_back_linkensure_removed {
() => {
// Module: crate::back::link
// Provides: {"ensure_removed"}
// Dependencies: {}
pub fn ensure_removed (dcx : DiagCtxtHandle < '_ > , path : & Path) { if let Err (e) = fs :: remove_file (path) { if e . kind () != io :: ErrorKind :: NotFound { dcx . err (format ! ("failed to remove {}: {}" , path . display () , e)) ; } } }
};
}
