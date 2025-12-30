// Generated macro for parse_char_specifiers (function)
macro_rules! Depcrateparse_char_specifiers {
() => {
// Module: crate
// Provides: {"parse_char_specifiers"}
// Dependencies: {}
fn parse_char_specifiers (s : & [char]) -> Vec < CharSpecifier > { let mut cs = Vec :: new () ; let mut i = 0 ; while i < s . len () { if i + 3 <= s . len () && s [i + 1] == '-' { cs . push (CharRange (s [i] , s [i + 2])) ; i += 3 ; } else { cs . push (SingleChar (s [i])) ; i += 1 ; } } cs }
};
}
