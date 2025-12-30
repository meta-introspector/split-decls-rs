// Generated macro for is_a_terminal (function)
macro_rules! Depcrate_windows_termis_a_terminal {
() => {
// Module: crate::windows_term
// Provides: {"is_a_terminal"}
// Dependencies: {}
pub (crate) fn is_a_terminal (out : & Term) -> bool { let (fd , others) = match out . target () { TermTarget :: Stdout => (STD_OUTPUT_HANDLE , [STD_INPUT_HANDLE , STD_ERROR_HANDLE]) , TermTarget :: Stderr => (STD_ERROR_HANDLE , [STD_INPUT_HANDLE , STD_OUTPUT_HANDLE]) , } ; if unsafe { console_on_any (& [fd]) } { return true ; } if unsafe { console_on_any (& others) } { return false ; } msys_tty_on (out) }
};
}
