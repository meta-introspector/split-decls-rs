// Generated macro for impl_701 (impl)
macro_rules! Depcrate_util_alphabetimpl_701 {
() => {
// Module: crate::util::alphabet
// Provides: {"impl_701"}
// Dependencies: {}
impl < 'a > Iterator for ByteSetIter < 'a > { type Item = u8 ; fn next (& mut self) -> Option < u8 > { while self . b <= 255 { let b = u8 :: try_from (self . b) . unwrap () ; self . b += 1 ; if self . set . contains (b) { return Some (b) ; } } None } }
};
}
