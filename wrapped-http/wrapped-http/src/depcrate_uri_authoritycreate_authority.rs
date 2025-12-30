// Generated macro for create_authority (function)
macro_rules! Depcrate_uri_authoritycreate_authority {
() => {
// Module: crate::uri::authority
// Provides: {"create_authority"}
// Dependencies: {}
fn create_authority < B , F > (b : B , f : F) -> Result < Authority , InvalidUri > where B : AsRef < [u8] > , F : FnOnce (B) -> Bytes , { let s = b . as_ref () ; let authority_end = Authority :: parse_non_empty (s) ? ; if authority_end != s . len () { return Err (ErrorKind :: InvalidUriChar . into ()) ; } let bytes = f (b) ; Ok (Authority { data : unsafe { ByteStr :: from_utf8_unchecked (bytes) } , }) }
};
}
