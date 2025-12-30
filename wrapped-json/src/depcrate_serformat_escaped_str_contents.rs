// Generated macro for format_escaped_str_contents (function)
macro_rules! Depcrate_serformat_escaped_str_contents {
() => {
// Module: crate::ser
// Provides: {"format_escaped_str_contents"}
// Dependencies: {}
fn format_escaped_str_contents < W , F > (writer : & mut W , formatter : & mut F , value : & str ,) -> io :: Result < () > where W : ? Sized + io :: Write , F : ? Sized + Formatter , { let mut bytes = value . as_bytes () ; let mut i = 0 ; while i < bytes . len () { let (string_run , rest) = bytes . split_at (i) ; let (& byte , rest) = rest . split_first () . unwrap () ; let escape = ESCAPE [byte as usize] ; i += 1 ; if escape == 0 { continue ; } bytes = rest ; i = 0 ; let string_run = unsafe { str :: from_utf8_unchecked (string_run) } ; if ! string_run . is_empty () { tri ! (formatter . write_string_fragment (writer , string_run)) ; } let char_escape = match escape { self :: BB => CharEscape :: Backspace , self :: TT => CharEscape :: Tab , self :: NN => CharEscape :: LineFeed , self :: FF => CharEscape :: FormFeed , self :: RR => CharEscape :: CarriageReturn , self :: QU => CharEscape :: Quote , self :: BS => CharEscape :: ReverseSolidus , self :: UU => CharEscape :: AsciiControl (byte) , _ => unsafe { hint :: unreachable_unchecked () } , } ; tri ! (formatter . write_char_escape (writer , char_escape)) ; } let string_run = unsafe { str :: from_utf8_unchecked (bytes) } ; if string_run . is_empty () { return Ok (()) ; } formatter . write_string_fragment (writer , string_run) }
};
}
