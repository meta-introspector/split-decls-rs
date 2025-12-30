// Generated macro for impl_54 (impl)
macro_rules! Depcrate_provider_dataimpl_54 {
() => {
// Module: crate::provider::data
// Provides: {"impl_54"}
// Dependencies: {}
impl TrieValue for CaseMapData { type TryFromU32Error = TryFromIntError ; fn try_from_u32 (i : u32) -> Result < Self , Self :: TryFromU32Error > { u16 :: try_from (i) . map (| u | AsULE :: from_unaligned (CaseMapDataULE (u . to_unaligned ()))) } fn to_u32 (self) -> u32 { u32 :: from (self . to_unaligned () . 0 . as_unsigned_int ()) } }
};
}
