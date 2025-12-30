// Generated macro for open (function)
macro_rules! Depcrate_windowsopen {
() => {
// Module: crate::windows
// Provides: {"open"}
// Dependencies: {}
pub (crate) fn open (path : & OsStr) -> Result < () , OpenError > { let Err (first_error) = open_helper (path) else { return Ok (()) ; } ; match PathBuf :: from (path) . normalize () { Ok (normalized) => match open_helper (normalized . as_os_str ()) { Ok (()) => Ok (()) , Err (_second_error) => Err (first_error) , } , Err (_) => Err (first_error) , } }
};
}
