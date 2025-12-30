// Generated macro for impl_759 (impl)
macro_rules! Depcrate_frameimpl_759 {
() => {
// Module: crate::frame
// Provides: {"impl_759"}
// Dependencies: {}
impl Iterator for AckIter < '_ > { type Item = RangeInclusive < u64 > ; fn next (& mut self) -> Option < RangeInclusive < u64 > > { if ! self . data . has_remaining () { return None ; } let block = self . data . get_var () . unwrap () ; let largest = self . largest ; if let Ok (gap) = self . data . get_var () { self . largest -= block + gap + 2 ; } Some (largest - block ..= largest) } }
};
}
