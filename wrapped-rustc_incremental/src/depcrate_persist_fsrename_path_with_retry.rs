// Generated macro for rename_path_with_retry (function)
macro_rules! Depcrate_persist_fsrename_path_with_retry {
() => {
// Module: crate::persist::fs
// Provides: {"rename_path_with_retry"}
// Dependencies: {}
fn rename_path_with_retry (from : & Path , to : & Path , mut retries_left : usize) -> std :: io :: Result < () > { loop { match std_fs :: rename (from , to) { Ok (()) => return Ok (()) , Err (e) => { if retries_left > 0 && e . kind () == ErrorKind :: PermissionDenied { std :: thread :: sleep (Duration :: from_millis (50)) ; retries_left -= 1 ; } else { return Err (e) ; } } } } }
};
}
