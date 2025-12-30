// Generated macro for impl_435 (impl)
macro_rules! Depcrate_trievalueimpl_435 {
() => {
// Module: crate::trievalue
// Provides: {"impl_435"}
// Dependencies: {}
impl TrieValue for WordBreak { type TryFromU32Error = TryFromIntError ; fn try_from_u32 (i : u32) -> Result < Self , Self :: TryFromU32Error > { u8 :: try_from (i) . map (Self) } fn to_u32 (self) -> u32 { u32 :: from (self . 0) } }
};
}
