// Generated macro for impl_104 (impl)
macro_rules! Depcrate_nfa_noncontiguousimpl_104 {
() => {
// Module: crate::nfa::noncontiguous
// Provides: {"impl_104"}
// Dependencies: {}
impl Transition { # [doc = " Return the byte for which this transition is defined."] pub (crate) fn byte (& self) -> u8 { self . byte } # [doc = " Return the ID of the state that this transition points to."] pub (crate) fn next (& self) -> StateID { self . next } # [doc = " Return the ID of the next transition."] fn link (& self) -> StateID { self . link } }
};
}
