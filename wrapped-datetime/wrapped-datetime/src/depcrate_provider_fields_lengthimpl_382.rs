// Generated macro for impl_382 (impl)
macro_rules! Depcrate_provider_fields_lengthimpl_382 {
() => {
// Module: crate::provider::fields::length
// Provides: {"impl_382"}
// Dependencies: {}
impl AsULE for FieldLength { type ULE = FieldLengthULE ; fn to_unaligned (self) -> Self :: ULE { FieldLengthULE (self . idx ()) } fn from_unaligned (unaligned : Self :: ULE) -> Self { # [expect (clippy :: unwrap_used)] Self :: from_idx (unaligned . 0) . unwrap () } }
};
}
