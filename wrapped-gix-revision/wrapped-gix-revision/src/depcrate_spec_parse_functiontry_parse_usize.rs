// Generated macro for try_parse_usize (function)
macro_rules! Depcrate_spec_parse_functiontry_parse_usize {
() => {
// Module: crate::spec::parse::function
// Provides: {"try_parse_usize"}
// Dependencies: {}
fn try_parse_usize (input : & BStr) -> Result < Option < (usize , usize) > , Error > { let mut bytes = input . iter () . peekable () ; if bytes . peek () . filter (| & & & b | b == b'-' || b == b'+') . is_some () { return Err (Error :: SignedNumber { input : input . into () }) ; } let num_digits = bytes . take_while (| b | b . is_ascii_digit ()) . count () ; if num_digits == 0 { return Ok (None) ; } let input = & input [.. num_digits] ; let number = try_parse (input) ? . ok_or_else (| | Error :: InvalidNumber { input : input . into () }) ? ; Ok (Some ((number , num_digits))) }
};
}
