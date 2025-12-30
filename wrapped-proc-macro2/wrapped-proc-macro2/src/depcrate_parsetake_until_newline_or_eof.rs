// Generated macro for take_until_newline_or_eof (function)
macro_rules! Depcrate_parsetake_until_newline_or_eof {
() => {
// Module: crate::parse
// Provides: {"take_until_newline_or_eof"}
// Dependencies: {}
fn take_until_newline_or_eof (input : Cursor) -> (Cursor , & str) { let chars = input . char_indices () ; for (i , ch) in chars { if ch == '\n' { return (input . advance (i) , & input . rest [.. i]) ; } else if ch == '\r' && input . rest [i + 1 ..] . starts_with ('\n') { return (input . advance (i + 1) , & input . rest [.. i]) ; } } (input . advance (input . len ()) , input . rest) }
};
}
