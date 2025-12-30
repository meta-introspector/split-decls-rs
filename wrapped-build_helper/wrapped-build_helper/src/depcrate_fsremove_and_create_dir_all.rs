// Generated macro for remove_and_create_dir_all (function)
macro_rules! Depcrate_fsremove_and_create_dir_all {
() => {
// Module: crate::fs
// Provides: {"remove_and_create_dir_all"}
// Dependencies: {}
pub fn remove_and_create_dir_all < P : AsRef < Path > > (path : P) -> io :: Result < () > { let path = path . as_ref () ; recursive_remove (path) ? ; fs :: create_dir_all (path) }
};
}
