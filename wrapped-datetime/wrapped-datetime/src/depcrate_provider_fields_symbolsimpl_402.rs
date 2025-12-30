// Generated macro for impl_402 (impl)
macro_rules! Depcrate_provider_fields_symbolsimpl_402 {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"impl_402"}
// Dependencies: {}
impl AsULE for FieldSymbol { type ULE = FieldSymbolULE ; fn to_unaligned (self) -> Self :: ULE { FieldSymbolULE (self . idx ()) } fn from_unaligned (unaligned : Self :: ULE) -> Self { # [expect (clippy :: unwrap_used)] Self :: from_idx (unaligned . 0) . unwrap () } }
};
}
