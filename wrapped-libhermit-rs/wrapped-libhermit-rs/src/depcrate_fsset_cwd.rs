// Generated macro for set_cwd (function)
macro_rules! Depcrate_fsset_cwd {
() => {
// Module: crate::fs
// Provides: {"set_cwd"}
// Dependencies: {}
pub fn set_cwd (cwd : & str) -> io :: Result < () > { let mut working_dir = WORKING_DIRECTORY . lock () ; if cwd . starts_with ("/") { * working_dir = Some (cwd . to_string ()) ; } else { let Some (working_dir) = working_dir . as_mut () else { return Err (Errno :: Badf) ; } ; working_dir . push ('/') ; working_dir . push_str (cwd) ; } Ok (()) }
};
}
