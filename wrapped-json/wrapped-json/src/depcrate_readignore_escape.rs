// Generated macro for ignore_escape (function)
macro_rules! Depcrate_readignore_escape {
() => {
// Module: crate::read
// Provides: {"ignore_escape"}
// Dependencies: {}
# [doc = " Parses a JSON escape sequence and discards the value. Assumes the previous"] # [doc = " byte read was a backslash."] fn ignore_escape < 'de , R > (read : & mut R) -> Result < () > where R : ? Sized + Read < 'de > , { let ch = tri ! (next_or_eof (read)) ; match ch { b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => { } b'u' => { tri ! (read . decode_hex_escape ()) ; } _ => { return error (read , ErrorCode :: InvalidEscape) ; } } Ok (()) }
};
}
