// Generated macro for truncate (function)
macro_rules! Depcrate_fstruncate {
() => {
// Module: crate::fs
// Provides: {"truncate"}
// Dependencies: {}
pub fn truncate (name : & str , size : usize) -> io :: Result < () > { with_relative_filename (name , | name | { let fs = FILESYSTEM . get () . ok_or (Errno :: Inval) ? ; if let Ok (file) = fs . open (name , OpenOption :: O_TRUNC , AccessPermission :: empty ()) { block_on (async { file . read () . await . truncate (size) . await } , None) } else { Err (Errno :: Badf) } }) }
};
}
