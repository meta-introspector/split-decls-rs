// Generated macro for impl_356 (impl)
macro_rules! Depcrate_util_alphabetimpl_356 {
() => {
// Module: crate::util::alphabet
// Provides: {"impl_356"}
// Dependencies: {}
impl < 'a > Iterator for ByteClassElements < 'a > { type Item = u8 ; fn next (& mut self) -> Option < u8 > { while let Some (byte) = self . bytes . next () { if self . class == self . classes . get (byte) { return Some (byte) ; } } None } }
};
}
