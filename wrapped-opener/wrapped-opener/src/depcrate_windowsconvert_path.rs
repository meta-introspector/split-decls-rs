// Generated macro for convert_path (function)
macro_rules! Depcrate_windowsconvert_path {
() => {
// Module: crate::windows
// Provides: {"convert_path"}
// Dependencies: {}
fn convert_path (path : & OsStr) -> io :: Result < Vec < u16 > > { let mut maybe_result : Vec < u16 > = path . encode_wide () . collect () ; if maybe_result . contains (& 0) { return Err (io :: Error :: new (io :: ErrorKind :: InvalidInput , "path contains NUL byte(s)" ,)) ; } maybe_result . push (0) ; Ok (maybe_result) }
};
}
