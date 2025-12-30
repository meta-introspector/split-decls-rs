// Generated macro for NON_ALPHANUMERIC (const)
macro_rules! Depcrate_ascii_setNON_ALPHANUMERIC {
() => {
// Module: crate::ascii_set
// Provides: {"NON_ALPHANUMERIC"}
// Dependencies: {}
# [doc = " Everything that is not an ASCII letter or digit."] # [doc = ""] # [doc = " This is probably more eager than necessary in any context."] pub const NON_ALPHANUMERIC : & AsciiSet = & CONTROLS . add (b' ') . add (b'!') . add (b'"') . add (b'#') . add (b'$') . add (b'%') . add (b'&') . add (b'\'') . add (b'(') . add (b')') . add (b'*') . add (b'+') . add (b',') . add (b'-') . add (b'.') . add (b'/') . add (b':') . add (b';') . add (b'<') . add (b'=') . add (b'>') . add (b'?') . add (b'@') . add (b'[') . add (b'\\') . add (b']') . add (b'^') . add (b'_') . add (b'`') . add (b'{') . add (b'|') . add (b'}') . add (b'~') ;
};
}
