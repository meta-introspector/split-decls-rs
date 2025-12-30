// Generated macro for parse_token (function)
macro_rules! Depcrateparse_token {
() => {
// Module: crate
// Provides: {"parse_token"}
// Dependencies: {}
# [inline] fn parse_token < 'a > (bytes : & mut Bytes < 'a >) -> Result < & 'a str > { let b = next ! (bytes) ; if ! is_method_token (b) { return Err (Error :: Token) ; } loop { let b = next ! (bytes) ; if b == b' ' { return Ok (Status :: Complete (unsafe { str :: from_utf8_unchecked (bytes . slice_skip (1)) } ,)) ; } else if ! is_method_token (b) { return Err (Error :: Token) ; } } }
};
}
