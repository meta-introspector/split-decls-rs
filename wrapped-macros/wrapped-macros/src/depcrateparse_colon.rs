// Generated macro for parse_colon (function)
macro_rules! Depcrateparse_colon {
() => {
// Module: crate
// Provides: {"parse_colon"}
// Dependencies: {}
# [doc = " parses the stuff after a `{:` into a [Piece] and the trailing `&str` (what comes after the `}`)"] fn parse_colon (format : & str , span : Span) -> parse :: Result < (Piece , & str) > { let (format , prefix) = if let Some (tail) = format . strip_prefix ('#') { (tail , true) } else { (format , false) } ; let (format , pad_char) = if let Some (tail) = format . strip_prefix ('0') { (tail , b'0') } else { (format , b' ') } ; let (format , pad_length) = if ! format . is_empty () && if let Some (ch) = format . chars () . next () { ch . is_ascii_digit () } else { false } { split_number (format) } else { (format , 0) } ; if let Some (tail) = format . strip_prefix ("x}") { Ok ((Piece :: Hex { upper_case : false , pad_char , pad_length , prefix , } , tail ,)) } else if let Some (tail) = format . strip_prefix ("X}") { Ok ((Piece :: Hex { upper_case : true , pad_char , pad_length , prefix , } , tail ,)) } else { Err (parse :: Error :: new (span , "invalid format string: expected `{{`, `{}`, `{:?}`, `{:#?}` or '{:x}'" ,)) } }
};
}
