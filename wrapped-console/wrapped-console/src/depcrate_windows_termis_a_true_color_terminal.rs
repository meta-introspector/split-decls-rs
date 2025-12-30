// Generated macro for is_a_true_color_terminal (function)
macro_rules! Depcrate_windows_termis_a_true_color_terminal {
() => {
// Module: crate::windows_term
// Provides: {"is_a_true_color_terminal"}
// Dependencies: {}
pub (crate) fn is_a_true_color_terminal (out : & Term) -> bool { if ! is_a_color_terminal (out) { return false ; } if msys_tty_on (out) { return match env :: var ("COLORTERM") { Ok (term) => term == "truecolor" || term == "24bit" , Err (_) => true , } ; } false }
};
}
