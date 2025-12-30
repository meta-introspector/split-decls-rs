// Generated macro for impl_67 (impl)
macro_rules! Depcrate_providerimpl_67 {
() => {
// Module: crate::provider
// Provides: {"impl_67"}
// Dependencies: {}
impl PluralCategoryAndMetadataPackedULE { fn unpack_byte (byte : u8) -> PluralCategoryAndMetadataUnpacked { let plural_category_byte = (byte & 0xF0) >> 4 ; let metadata_byte = byte & 0x0F ; PluralCategoryAndMetadataUnpacked { plural_category_byte , metadata_byte , } } fn get (self) -> PluralCategoryAndMetadata { let unpacked = Self :: unpack_byte (self . 0) ; unsafe { PluralCategoryAndMetadata :: try_from_unpacked (unpacked) . unwrap_unchecked () } } }
};
}
