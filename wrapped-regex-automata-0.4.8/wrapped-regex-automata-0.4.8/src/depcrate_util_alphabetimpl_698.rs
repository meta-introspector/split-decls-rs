// Generated macro for impl_698 (impl)
macro_rules! Depcrate_util_alphabetimpl_698 {
() => {
// Module: crate::util::alphabet
// Provides: {"impl_698"}
// Dependencies: {}
impl < 'a > Iterator for ByteSetIter < 'a > { type Item = u8 ; fn next (& mut self) -> Option < u8 > { while self . b <= 255 { let b = u8 :: try_from (self . b) . unwrap () ; self . b += 1 ; if self . set . contains (b) { return Some (b) ; } } None } }
};
}
