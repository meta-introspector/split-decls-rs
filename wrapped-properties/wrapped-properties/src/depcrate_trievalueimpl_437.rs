// Generated macro for impl_437 (impl)
macro_rules! Depcrate_trievalueimpl_437 {
() => {
// Module: crate::trievalue
// Provides: {"impl_437"}
// Dependencies: {}
impl TrieValue for IndicConjunctBreak { type TryFromU32Error = TryFromIntError ; fn try_from_u32 (i : u32) -> Result < Self , Self :: TryFromU32Error > { u8 :: try_from (i) . map (Self) } fn to_u32 (self) -> u32 { u32 :: from (self . 0) } }
};
}
