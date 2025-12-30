// Generated macro for impl_384 (impl)
macro_rules! Depcrate_provider_fields_lengthimpl_384 {
() => {
// Module: crate::provider::fields::length
// Provides: {"impl_384"}
// Dependencies: {}
unsafe impl ULE for FieldLengthULE { fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { for byte in bytes { Self :: validate_byte (* byte) ? ; } Ok (()) } }
};
}
