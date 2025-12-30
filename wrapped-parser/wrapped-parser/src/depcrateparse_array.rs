// Generated macro for parse_array (function)
macro_rules! Depcrateparse_array {
() => {
// Module: crate
// Provides: {"parse_array"}
// Dependencies: {}
# [doc = " Parse and consume an array at the beginning of `s`."] # [doc = ""] # [doc = " Return the length of the array."] fn parse_array (mut s : & str) -> Result < usize , Error > { let len_pos = s . find (| c : char | c != ' ') . ok_or (Error :: InvalidArraySpecifierMissingLength) ? ; s = & s [len_pos ..] ; let after_len = s . find (| c : char | ! c . is_ascii_digit ()) . ok_or (Error :: InvalidArraySpecifierMissingBracket) ? ; let len = s [.. after_len] . parse :: < usize > () ? ; s = & s [after_len ..] ; if s != "]" { return Err (Error :: InvalidArraySpecifierMissingBracket) ; } Ok (len) }
};
}
