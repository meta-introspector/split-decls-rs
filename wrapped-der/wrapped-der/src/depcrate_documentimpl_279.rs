// Generated macro for impl_279 (impl)
macro_rules! Depcrate_documentimpl_279 {
() => {
// Module: crate::document
// Provides: {"impl_279"}
// Dependencies: {}
impl TryFrom < Vec < u8 > > for Document { type Error = Error ; fn try_from (der_bytes : Vec < u8 >) -> Result < Self , Error > { let mut decoder = SliceReader :: new (& der_bytes) ? ; decode_sequence (& mut decoder) ? ; decoder . finish () ? ; let length = der_bytes . len () . try_into () ? ; Ok (Self { der_bytes , length }) } }
};
}
