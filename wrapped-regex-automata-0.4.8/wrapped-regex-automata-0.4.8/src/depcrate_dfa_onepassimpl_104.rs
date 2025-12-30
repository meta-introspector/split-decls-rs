// Generated macro for impl_104 (impl)
macro_rules! Depcrate_dfa_onepassimpl_104 {
() => {
// Module: crate::dfa::onepass
// Provides: {"impl_104"}
// Dependencies: {}
impl Iterator for SlotsIter { type Item = usize ; fn next (& mut self) -> Option < usize > { let slot = self . slots . 0 . trailing_zeros () . as_usize () ; if slot >= Slots :: LIMIT { return None ; } self . slots = self . slots . remove (slot) ; Some (slot) } }
};
}
