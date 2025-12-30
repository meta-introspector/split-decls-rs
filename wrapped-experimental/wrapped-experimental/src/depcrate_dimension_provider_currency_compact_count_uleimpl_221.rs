// Generated macro for impl_221 (impl)
macro_rules! Depcrate_dimension_provider_currency_compact_count_uleimpl_221 {
() => {
// Module: crate::dimension::provider::currency::compact_count_ule
// Provides: {"impl_221"}
// Dependencies: {}
unsafe impl ULE for CompactCountULE { fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { for byte in bytes { if byte & 0b0111_1000 != 0 { return Err (UleError :: parse :: < Self > ()) ; } if byte & 0b0000_0111 > 5 { return Err (UleError :: parse :: < Self > ()) ; } } Ok (()) } }
};
}
