// Generated macro for impl_275 (impl)
macro_rules! Depcrate_documentimpl_275 {
() => {
// Module: crate::document
// Provides: {"impl_275"}
// Dependencies: {}
impl < 'a > Decode < 'a > for Document { type Error = Error ; fn decode < R : Reader < 'a > > (reader : & mut R) -> Result < Document , Error > { let header = Header :: peek (reader) ? ; let length = (header . encoded_len () ? + header . length ()) ? ; let bytes = reader . read_slice (length) ? ; Ok (Self { der_bytes : bytes . into () , length , }) } }
};
}
