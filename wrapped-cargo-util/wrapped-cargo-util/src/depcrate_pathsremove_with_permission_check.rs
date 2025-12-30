// Generated macro for remove_with_permission_check (function)
macro_rules! Depcrate_pathsremove_with_permission_check {
() => {
// Module: crate::paths
// Provides: {"remove_with_permission_check"}
// Dependencies: {}
fn remove_with_permission_check < F , P > (remove_func : F , p : P) -> io :: Result < () > where F : Fn (P) -> io :: Result < () > , P : AsRef < Path > + Clone , { match remove_func (p . clone ()) { Ok (()) => Ok (()) , Err (e) => { if e . kind () == io :: ErrorKind :: PermissionDenied && set_not_readonly (p . as_ref ()) . unwrap_or (false) { remove_func (p) } else { Err (e) } } } }
};
}
