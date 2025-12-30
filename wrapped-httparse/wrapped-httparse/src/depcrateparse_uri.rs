// Generated macro for parse_uri (function)
macro_rules! Depcrateparse_uri {
() => {
// Module: crate
// Provides: {"parse_uri"}
// Dependencies: {}
# [inline] # [doc (hidden)] # [allow (missing_docs)] pub fn parse_uri < 'a > (bytes : & mut Bytes < 'a >) -> Result < & 'a str > { let start = bytes . pos () ; simd :: match_uri_vectored (bytes) ; let end = bytes . pos () ; if next ! (bytes) == b' ' { if end == start { return Err (Error :: Token) ; } match str :: from_utf8 (unsafe { bytes . slice_skip (1) }) { Ok (uri) => Ok (Status :: Complete (uri)) , Err (_) => Err (Error :: Token) , } } else { Err (Error :: Token) } }
};
}
