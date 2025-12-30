// Generated macro for clear_chars (function)
macro_rules! Depcrate_common_termclear_chars {
() => {
// Module: crate::common_term
// Provides: {"clear_chars"}
// Dependencies: {}
pub (crate) fn clear_chars (out : & Term , n : usize) -> io :: Result < () > { if n > 0 { out . write_str (& format ! ("\x1b[{n}D\x1b[0K")) } else { Ok (()) } }
};
}
