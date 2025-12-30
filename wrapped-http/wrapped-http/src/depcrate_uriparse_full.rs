// Generated macro for parse_full (function)
macro_rules! Depcrate_uriparse_full {
() => {
// Module: crate::uri
// Provides: {"parse_full"}
// Dependencies: {}
fn parse_full (mut s : Bytes) -> Result < Uri , InvalidUri > { let scheme = match Scheme2 :: parse (& s [..]) ? { Scheme2 :: None => Scheme2 :: None , Scheme2 :: Standard (p) => { let _ = s . split_to (p . len () + 3) ; Scheme2 :: Standard (p) } Scheme2 :: Other (n) => { let mut scheme = s . split_to (n + 3) ; let _ = scheme . split_off (n) ; let val = unsafe { ByteStr :: from_utf8_unchecked (scheme) } ; Scheme2 :: Other (Box :: new (val)) } } ; let authority_end = Authority :: parse (& s [..]) ? ; if scheme . is_none () { if authority_end != s . len () { return Err (ErrorKind :: InvalidFormat . into ()) ; } let authority = Authority { data : unsafe { ByteStr :: from_utf8_unchecked (s) } , } ; return Ok (Uri { scheme : scheme . into () , authority , path_and_query : PathAndQuery :: empty () , }) ; } if authority_end == 0 { return Err (ErrorKind :: InvalidFormat . into ()) ; } let authority = s . split_to (authority_end) ; let authority = Authority { data : unsafe { ByteStr :: from_utf8_unchecked (authority) } , } ; Ok (Uri { scheme : scheme . into () , authority , path_and_query : PathAndQuery :: from_shared (s) ? , }) }
};
}
