// Generated macro for parse_method (function)
macro_rules! Depcrateparse_method {
() => {
// Module: crate
// Provides: {"parse_method"}
// Dependencies: {}
# [inline] # [doc (hidden)] # [allow (missing_docs)] pub fn parse_method < 'a > (bytes : & mut Bytes < 'a >) -> Result < & 'a str > { const GET : [u8 ; 4] = * b"GET " ; const POST : [u8 ; 4] = * b"POST" ; match bytes . peek_n :: < [u8 ; 4] > (4) { Some (GET) => { let method = unsafe { bytes . advance (4) ; str :: from_utf8_unchecked (bytes . slice_skip (1)) } ; Ok (Status :: Complete (method)) } Some (POST) if unsafe { bytes . peek_ahead (4) } == Some (b' ') => { let method = unsafe { bytes . advance (5) ; str :: from_utf8_unchecked (bytes . slice_skip (1)) } ; Ok (Status :: Complete (method)) } _ => parse_token (bytes) , } }
};
}
