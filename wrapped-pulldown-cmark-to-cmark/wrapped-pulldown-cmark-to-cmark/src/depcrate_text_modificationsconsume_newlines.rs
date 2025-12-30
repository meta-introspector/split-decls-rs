// Generated macro for consume_newlines (function)
macro_rules! Depcrate_text_modificationsconsume_newlines {
() => {
// Module: crate::text_modifications
// Provides: {"consume_newlines"}
// Dependencies: {}
pub (crate) fn consume_newlines < F > (f : & mut F , s : & mut State < '_ >) -> fmt :: Result where F : fmt :: Write , { while s . newlines_before_start != 0 { s . newlines_before_start -= 1 ; write_padded_newline (f , s) ? ; } Ok (()) }
};
}
