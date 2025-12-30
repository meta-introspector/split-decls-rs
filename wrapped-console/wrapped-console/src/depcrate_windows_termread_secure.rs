// Generated macro for read_secure (function)
macro_rules! Depcrate_windows_termread_secure {
() => {
// Module: crate::windows_term
// Provides: {"read_secure"}
// Dependencies: {}
pub (crate) fn read_secure () -> io :: Result < String > { let mut rv = String :: new () ; loop { match read_single_key (false) ? { Key :: Enter => { break ; } Key :: Char ('\x08') => { if ! rv . is_empty () { let new_len = rv . len () - 1 ; rv . truncate (new_len) ; } } Key :: Char (c) => { rv . push (c) ; } _ => { } } } Ok (rv) }
};
}
