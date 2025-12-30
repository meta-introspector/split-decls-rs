// Generated macro for read_version (function)
macro_rules! Depcrate_fsread_version {
() => {
// Module: crate::fs
// Provides: {"read_version"}
// Dependencies: {}
pub fn read_version () -> io :: Result < () > { eprintln ! () ; let path = "/proc/version" ; eprint ! ("{path} contains") ; let version = fs :: read_to_string (path) . unwrap () ; eprintln ! (" {version:?}") ; Ok (()) }
};
}
