// Generated macro for set_not_readonly (function)
macro_rules! Depcrate_pathsset_not_readonly {
() => {
// Module: crate::paths
// Provides: {"set_not_readonly"}
// Dependencies: {}
fn set_not_readonly (p : & Path) -> io :: Result < bool > { let mut perms = p . metadata () ? . permissions () ; if ! perms . readonly () { return Ok (false) ; } perms . set_readonly (false) ; fs :: set_permissions (p , perms) ? ; Ok (true) }
};
}
