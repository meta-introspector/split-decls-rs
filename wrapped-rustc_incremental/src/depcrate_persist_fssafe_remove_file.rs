// Generated macro for safe_remove_file (function)
macro_rules! Depcrate_persist_fssafe_remove_file {
() => {
// Module: crate::persist::fs
// Provides: {"safe_remove_file"}
// Dependencies: {}
fn safe_remove_file (p : & Path) -> io :: Result < () > { match std_fs :: remove_file (p) { Err (err) if err . kind () == io :: ErrorKind :: NotFound => Ok (()) , result => result , } }
};
}
