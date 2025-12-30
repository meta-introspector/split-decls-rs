// Generated macro for do_op (function)
macro_rules! Depcrate_core_build_steps_cleando_op {
() => {
// Module: crate::core::build_steps::clean
// Provides: {"do_op"}
// Dependencies: {}
fn do_op < F > (path : & Path , desc : & str , mut f : F) where F : FnMut (& Path) -> io :: Result < () > , { match f (path) { Ok (()) => { } # [cfg (windows)] Err (ref e) if e . kind () == ErrorKind :: PermissionDenied => { let m = t ! (path . symlink_metadata ()) ; let mut p = m . permissions () ; # [expect (clippy :: permissions_set_readonly_false)] p . set_readonly (false) ; t ! (fs :: set_permissions (path , p)) ; f (path) . unwrap_or_else (| e | { if m . file_type () . is_symlink () && path . is_dir () && fs :: remove_dir (path) . is_ok () { return ; } panic ! ("failed to {} {}: {}" , desc , path . display () , e) ; }) ; } Err (e) => { panic ! ("failed to {} {}: {}" , desc , path . display () , e) ; } } }
};
}
