// Generated macro for impl_65 (impl)
macro_rules! Depcrate_providerimpl_65 {
() => {
// Module: crate::provider
// Provides: {"impl_65"}
// Dependencies: {}
impl From < PluralCategoryAndMetadata > for PluralCategoryAndMetadataPackedULE { fn from (value : PluralCategoryAndMetadata) -> Self { let byte = ((value . plural_category as u8) << 4) | value . metadata . get () ; debug_assert ! (PluralCategoryAndMetadata :: try_from_unpacked (Self :: unpack_byte (byte)) . is_some ()) ; Self (byte) } }
};
}
