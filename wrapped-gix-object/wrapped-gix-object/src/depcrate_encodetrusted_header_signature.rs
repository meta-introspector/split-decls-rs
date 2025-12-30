// Generated macro for trusted_header_signature (function)
macro_rules! Depcrate_encodetrusted_header_signature {
() => {
// Module: crate::encode
// Provides: {"trusted_header_signature"}
// Dependencies: {}
pub (crate) fn trusted_header_signature (name : & [u8] , value : & gix_actor :: SignatureRef < '_ > , out : & mut dyn io :: Write ,) -> io :: Result < () > { out . write_all (name) ? ; out . write_all (SPACE) ? ; value . write_to (out) ? ; out . write_all (NL) }
};
}
