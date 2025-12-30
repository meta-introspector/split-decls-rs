// Generated macro for parse_bool (function)
macro_rules! Depcrateparse_bool {
() => {
// Module: crate
// Provides: {"parse_bool"}
// Dependencies: {}
fn parse_bool (s : & str) -> Result < bool , Error > { if s . eq_ignore_ascii_case ("false") { Ok (false) } else if s . eq_ignore_ascii_case ("true") { Ok (true) } else { Err (eyre ! ("Not a valid boolean: {s}")) } }
};
}
