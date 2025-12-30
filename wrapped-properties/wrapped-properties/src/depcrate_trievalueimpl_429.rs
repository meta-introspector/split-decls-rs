// Generated macro for impl_429 (impl)
macro_rules! Depcrate_trievalueimpl_429 {
() => {
// Module: crate::trievalue
// Provides: {"impl_429"}
// Dependencies: {}
impl TrieValue for Script { type TryFromU32Error = TryFromIntError ; fn try_from_u32 (i : u32) -> Result < Self , Self :: TryFromU32Error > { u16 :: try_from (i) . map (Script) } fn to_u32 (self) -> u32 { u32 :: from (self . 0) } }
};
}
