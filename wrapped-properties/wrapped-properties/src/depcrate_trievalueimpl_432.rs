// Generated macro for impl_432 (impl)
macro_rules! Depcrate_trievalueimpl_432 {
() => {
// Module: crate::trievalue
// Provides: {"impl_432"}
// Dependencies: {}
impl TrieValue for EastAsianWidth { type TryFromU32Error = TryFromIntError ; fn try_from_u32 (i : u32) -> Result < Self , Self :: TryFromU32Error > { u8 :: try_from (i) . map (Self) } fn to_u32 (self) -> u32 { u32 :: from (self . 0) } }
};
}
