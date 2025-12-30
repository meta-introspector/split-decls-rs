// Generated macro for impl_83 (impl)
macro_rules! Depcrate_store_impls_dynamic_typesimpl_83 {
() => {
// Module: crate::store_impls::dynamic::types
// Provides: {"impl_83"}
// Dependencies: {}
impl PackId { # [doc = " Returns the maximum of indices we can represent."] pub (crate) const fn max_indices () -> usize { (1 << 15) - 1 } # [doc = " Returns the maximum of packs we can represent if stored in a multi-index."] pub (crate) const fn max_packs_in_multi_index () -> gix_pack :: multi_index :: PackIndex { (1 << 16) - 1 } # [doc = " Packs have a built-in identifier to make data structures simpler, and this method represents ourselves as such id"] # [doc = " to be convertible back and forth. We essentially compress ourselves into a u32."] # [doc = ""] # [doc = " Bit 16 is a marker to tell us if it's a multi-pack or not, the ones before are the index file itself, the ones after"] # [doc = " are used to encode the pack index within the multi-pack."] pub (crate) fn to_intrinsic_pack_id (self) -> gix_pack :: data :: Id { assert ! (self . index < (1 << 15) , "There shouldn't be more than 2^15 indices") ; match self . multipack_index { None => self . index as gix_pack :: data :: Id , Some (midx) => { assert ! (midx <= Self :: max_packs_in_multi_index () , "There shouldn't be more than 2^16 packs per multi-index") ; (self . index as gix_pack :: data :: Id | (1 << 15)) | (midx << 16) as gix_pack :: data :: Id } } } pub (crate) fn from_intrinsic_pack_id (pack_id : gix_pack :: data :: Id) -> Self { if pack_id & (1 << 15) == 0 { PackId { index : (pack_id & 0x7fff) as IndexId , multipack_index : None , } } else { PackId { index : (pack_id & 0x7fff) as IndexId , multipack_index : Some (pack_id >> 16) , } } } }
};
}
