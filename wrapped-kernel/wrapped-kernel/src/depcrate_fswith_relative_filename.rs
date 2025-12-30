// Generated macro for with_relative_filename (function)
macro_rules! Depcrate_fswith_relative_filename {
() => {
// Module: crate::fs
// Provides: {"with_relative_filename"}
// Dependencies: {}
fn with_relative_filename < F , T > (name : & str , callback : F) -> io :: Result < T > where F : FnOnce (& str) -> io :: Result < T > , { if name . starts_with ("/") { callback (name) } else { let cwd = WORKING_DIRECTORY . lock () ; if let Some (cwd) = cwd . as_ref () { let mut path = String :: with_capacity (cwd . len () + name . len () + 1) ; path . push_str (cwd) ; path . push ('/') ; path . push_str (name) ; callback (& path) } else { Err (Errno :: Badf) } } }
};
}
