// Generated macro for impl_268 (impl)
macro_rules! Depcrate_dimension_provider_currency_uleimpl_268 {
() => {
// Module: crate::dimension::provider::currency::ule
// Provides: {"impl_268"}
// Dependencies: {}
unsafe impl ULE for CurrencyPatternConfigULE { fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { if bytes . len () % 3 != 0 { return Err (UleError :: length :: < Self > (bytes . len ())) ; } Ok (()) } }
};
}
