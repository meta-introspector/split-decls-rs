// Generated macro for dump_buffer_into_formatter (function)
macro_rules! Depcrate_provider_pattern_runtime_displaydump_buffer_into_formatter {
() => {
// Module: crate::provider::pattern::runtime::display
// Provides: {"dump_buffer_into_formatter"}
// Dependencies: {}
# [doc = " A helper function optimized to dump string buffers into `Pattern`"] # [doc = " serialization wrapping minimal chunks of the buffer in escaping `'`"] # [doc = " literals to produce valid UTF35 pattern string."] fn dump_buffer_into_formatter < W : Write + ? Sized > (literal : & str , formatter : & mut W) -> fmt :: Result { if literal . is_empty () { return Ok (()) ; } let mut needs_escaping = false ; for ch in literal . chars () { if ch . is_ascii_alphabetic () || ch == '\'' { needs_escaping = true ; break ; } } if needs_escaping { let mut ch_iter = literal . trim_end () . chars () . peekable () ; while let Some (ch) = ch_iter . peek () { if ch . is_whitespace () { formatter . write_char (* ch) ? ; ch_iter . next () ; } else { break ; } } formatter . write_char ('\'') ? ; for ch in ch_iter { if ch == '\'' { formatter . write_char ('\\') ? ; } formatter . write_char (ch) ? ; } formatter . write_char ('\'') ? ; for ch in literal . chars () . rev () { if ch . is_whitespace () { formatter . write_char (ch) ? ; } else { break ; } } } else { formatter . write_str (literal) ? ; } Ok (()) }
};
}
