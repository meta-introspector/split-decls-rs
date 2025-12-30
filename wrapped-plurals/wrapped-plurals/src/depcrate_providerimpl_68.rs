// Generated macro for impl_68 (impl)
macro_rules! Depcrate_providerimpl_68 {
() => {
// Module: crate::provider
// Provides: {"impl_68"}
// Dependencies: {}
impl PluralCategoryAndMetadata { fn try_from_unpacked (unpacked : PluralCategoryAndMetadataUnpacked) -> Option < Self > { let plural_category = PluralElementsKeys :: new_from_u8 (unpacked . plural_category_byte) ? ; let metadata = FourBitMetadata :: try_from_byte (unpacked . metadata_byte) ? ; Some (Self { plural_category , metadata , }) } }
};
}
