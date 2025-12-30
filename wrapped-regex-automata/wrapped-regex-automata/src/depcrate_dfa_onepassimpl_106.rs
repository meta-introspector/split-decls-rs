// Generated macro for impl_106 (impl)
macro_rules! Depcrate_dfa_onepassimpl_106 {
() => {
// Module: crate::dfa::onepass
// Provides: {"impl_106"}
// Dependencies: {}
impl Iterator for SlotsIter { type Item = usize ; fn next (& mut self) -> Option < usize > { let slot = self . slots . 0 . trailing_zeros () . as_usize () ; if slot >= Slots :: LIMIT { return None ; } self . slots = self . slots . remove (slot) ; Some (slot) } }
};
}
