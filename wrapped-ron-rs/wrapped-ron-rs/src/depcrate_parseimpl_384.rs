// Generated macro for impl_384 (impl)
macro_rules! Depcrate_parseimpl_384 {
() => {
// Module: crate::parse
// Provides: {"impl_384"}
// Dependencies: {}
impl < 'a > ParsedStr < 'a > { pub fn try_from_bytes (bytes : ParsedByteStr < 'a >) -> Result < Self , Utf8Error > { match bytes { ParsedByteStr :: Allocated (byte_buf) => Ok (ParsedStr :: Allocated (String :: from_utf8 (byte_buf) . map_err (| e | e . utf8_error ()) ? ,)) , ParsedByteStr :: Slice (bytes) => Ok (ParsedStr :: Slice (from_utf8 (bytes) ?)) , } } }
};
}
