// Generated macro for trusted_header_id (function)
macro_rules! Depcrate_encodetrusted_header_id {
() => {
// Module: crate::encode
// Provides: {"trusted_header_id"}
// Dependencies: {}
pub (crate) fn trusted_header_id (name : & [u8] , value : & gix_hash :: ObjectId , mut out : & mut dyn io :: Write ,) -> io :: Result < () > { out . write_all (name) ? ; out . write_all (SPACE) ? ; value . write_hex_to (& mut out) ? ; out . write_all (NL) }
};
}
