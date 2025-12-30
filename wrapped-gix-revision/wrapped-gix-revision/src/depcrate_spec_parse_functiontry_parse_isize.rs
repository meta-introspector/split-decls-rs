// Generated macro for try_parse_isize (function)
macro_rules! Depcrate_spec_parse_functiontry_parse_isize {
() => {
// Module: crate::spec::parse::function
// Provides: {"try_parse_isize"}
// Dependencies: {}
fn try_parse_isize (input : & BStr) -> Result < Option < (isize , bool , usize) > , Error > { let mut bytes = input . iter () . peekable () ; if bytes . peek () . filter (| & & & b | b == b'+') . is_some () { return Err (Error :: SignedNumber { input : input . into () }) ; } let negative = bytes . peek () == Some (& & b'-') ; let num_digits = bytes . take_while (| b | b . is_ascii_digit () || * b == & b'-') . count () ; if num_digits == 0 { return Ok (None) ; } else if num_digits == 1 && negative { return Ok (Some ((- 1 , negative , num_digits))) ; } let input = & input [.. num_digits] ; let number = try_parse (input) ? . ok_or_else (| | Error :: InvalidNumber { input : input . into () }) ? ; Ok (Some ((number , negative , num_digits))) }
};
}
