// Generated macro for impl_431 (impl)
macro_rules! Depcrate_trievalueimpl_431 {
() => {
// Module: crate::trievalue
// Provides: {"impl_431"}
// Dependencies: {}
impl TrieValue for ScriptWithExt { type TryFromU32Error = TryFromIntError ; fn try_from_u32 (i : u32) -> Result < Self , Self :: TryFromU32Error > { u16 :: try_from (i) . map (Self) } fn to_u32 (self) -> u32 { u32 :: from (self . 0) } }
};
}
