// Generated macro for impl_443 (impl)
macro_rules! Depcrate_trievalueimpl_443 {
() => {
// Module: crate::trievalue
// Provides: {"impl_443"}
// Dependencies: {}
impl TrieValue for GeneralCategoryGroup { type TryFromU32Error = TryFromIntError ; fn try_from_u32 (i : u32) -> Result < Self , Self :: TryFromU32Error > { u16 :: try_from (i) . map (packed_u16_to_gcg) } fn to_u32 (self) -> u32 { u32 :: from (gcg_to_packed_u16 (self)) } }
};
}
