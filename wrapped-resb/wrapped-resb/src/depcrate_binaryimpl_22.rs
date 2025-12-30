// Generated macro for impl_22 (impl)
macro_rules! Depcrate_binaryimpl_22 {
() => {
// Module: crate::binary
// Provides: {"impl_22"}
// Dependencies: {}
impl BinaryDeserializerError { # [doc = " TODO"] pub fn invalid_data (message : & 'static str) -> Self { Self { kind : ErrorKind :: InvalidData , message , } } # [doc = " TODO"] pub fn resource_type_mismatch (message : & 'static str) -> Self { Self { kind : ErrorKind :: ResourceTypeMismatch , message , } } # [doc = " TODO"] pub fn unsupported_format (message : & 'static str) -> Self { Self { kind : ErrorKind :: UnsupportedFormat , message , } } # [doc = " TODO"] pub fn unknown (message : & 'static str) -> Self { Self { kind : ErrorKind :: Unknown , message , } } }
};
}
