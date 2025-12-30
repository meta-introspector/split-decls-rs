// Generated macro for impl_445 (impl)
macro_rules! Depcrate_trievalueimpl_445 {
() => {
// Module: crate::trievalue
// Provides: {"impl_445"}
// Dependencies: {}
impl TrieValue for JoiningType { type TryFromU32Error = TryFromIntError ; fn try_from_u32 (i : u32) -> Result < Self , Self :: TryFromU32Error > { u8 :: try_from (i) . map (Self) } fn to_u32 (self) -> u32 { u32 :: from (self . 0) } }
};
}
