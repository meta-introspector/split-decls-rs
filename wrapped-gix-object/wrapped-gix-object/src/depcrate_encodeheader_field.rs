// Generated macro for header_field (function)
macro_rules! Depcrate_encodeheader_field {
() => {
// Module: crate::encode
// Provides: {"header_field"}
// Dependencies: {}
pub (crate) fn header_field (name : & [u8] , value : & [u8] , out : & mut dyn io :: Write) -> io :: Result < () > { if value . is_empty () { return Err (Error :: EmptyValue . into ()) ; } if value . find (NL) . is_some () { return Err (Error :: NewlineInHeaderValue { value : value . into () } . into ()) ; } trusted_header_field (name , value , out) }
};
}
