// Generated macro for impl_404 (impl)
macro_rules! Depcrate_provider_fields_symbolsimpl_404 {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"impl_404"}
// Dependencies: {}
unsafe impl ULE for FieldSymbolULE { fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { for byte in bytes { Self :: validate_byte (* byte) ? ; } Ok (()) } }
};
}
