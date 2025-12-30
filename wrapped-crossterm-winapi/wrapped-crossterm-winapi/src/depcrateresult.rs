// Generated macro for result (function)
macro_rules! Depcrateresult {
() => {
// Module: crate
// Provides: {"result"}
// Dependencies: {}
# [doc = " Get the result of a call to WinAPI as an [`io::Result`]."] # [inline] pub fn result (return_value : BOOL) -> io :: Result < () > { if return_value != 0 { Ok (()) } else { Err (io :: Error :: last_os_error ()) } }
};
}
