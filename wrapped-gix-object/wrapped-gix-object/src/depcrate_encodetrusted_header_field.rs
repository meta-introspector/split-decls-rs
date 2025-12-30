// Generated macro for trusted_header_field (function)
macro_rules! Depcrate_encodetrusted_header_field {
() => {
// Module: crate::encode
// Provides: {"trusted_header_field"}
// Dependencies: {}
pub (crate) fn trusted_header_field (name : & [u8] , value : & [u8] , out : & mut dyn io :: Write) -> io :: Result < () > { out . write_all (name) ? ; out . write_all (SPACE) ? ; out . write_all (value) ? ; out . write_all (NL) }
};
}
