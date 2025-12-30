// Generated macro for get_cwd (function)
macro_rules! Depcrate_fsget_cwd {
() => {
// Module: crate::fs
// Provides: {"get_cwd"}
// Dependencies: {}
pub fn get_cwd () -> io :: Result < String > { let cwd = WORKING_DIRECTORY . lock () ; if let Some (cwd) = cwd . as_ref () { Ok (cwd . clone ()) } else { Err (Errno :: Noent) } }
};
}
