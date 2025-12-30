// Generated macro for impl_264 (impl)
macro_rules! Depcrate_providerimpl_264 {
() => {
// Module: crate::provider
// Provides: {"impl_264"}
// Dependencies: {}
impl CollationReordering < '_ > { pub (crate) fn reorder (& self , primary : u32) -> u32 { if let Some (b) = self . reorder_table . get ((primary >> 24) as usize) { if b != 0 || primary <= NO_CE_PRIMARY { (u32 :: from (b) << 24) | (primary & 0x00FFFFFF) } else { self . reorder_ex (primary) } } else { debug_assert ! (false) ; primary } } fn reorder_ex (& self , primary : u32) -> u32 { if primary >= self . min_high_no_reorder { return primary ; } let q = primary | 0xFFFF ; for & range in self . reorder_ranges . as_ule_slice () . iter () { let r = u32 :: from_unaligned (range) ; if q < r { return primary . wrapping_add (r << 24) ; } } debug_assert ! (false) ; primary } }
};
}
