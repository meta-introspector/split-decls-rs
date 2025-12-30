// Generated macro for impl_66 (impl)
macro_rules! Depcrate_providerimpl_66 {
() => {
// Module: crate::provider
// Provides: {"impl_66"}
// Dependencies: {}
unsafe impl ULE for PluralCategoryAndMetadataPackedULE { fn validate_bytes (bytes : & [u8]) -> Result < () , zerovec :: ule :: UleError > { bytes . iter () . all (| byte | { let unpacked = Self :: unpack_byte (* byte) ; PluralCategoryAndMetadata :: try_from_unpacked (unpacked) . is_some () }) . then_some (()) . ok_or_else (UleError :: parse :: < Self >) } }
};
}
