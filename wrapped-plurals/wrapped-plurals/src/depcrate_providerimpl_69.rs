// Generated macro for impl_69 (impl)
macro_rules! Depcrate_providerimpl_69 {
() => {
// Module: crate::provider
// Provides: {"impl_69"}
// Dependencies: {}
impl AsULE for PluralCategoryAndMetadata { type ULE = PluralCategoryAndMetadataPackedULE ; # [inline] fn to_unaligned (self) -> Self :: ULE { PluralCategoryAndMetadataPackedULE :: from (self) } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { unaligned . get () } }
};
}
