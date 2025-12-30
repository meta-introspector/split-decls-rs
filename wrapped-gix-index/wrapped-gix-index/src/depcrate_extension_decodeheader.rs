// Generated macro for header (function)
macro_rules! Depcrate_extension_decodeheader {
() => {
// Module: crate::extension::decode
// Provides: {"header"}
// Dependencies: {}
pub (crate) fn header (data : & [u8]) -> (Signature , u32 , & [u8]) { let (signature , data) = data . split_at (4) ; let (size , data) = data . split_at (4) ; (signature . try_into () . unwrap () , from_be_u32 (size) , data) }
};
}
