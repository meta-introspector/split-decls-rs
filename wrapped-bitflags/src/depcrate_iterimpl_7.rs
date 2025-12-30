// Generated macro for impl_7 (impl)
macro_rules! Depcrate_iterimpl_7 {
() => {
// Module: crate::iter
// Provides: {"impl_7"}
// Dependencies: {}
impl < B : Flags > Iterator for Iter < B > { type Item = B ; fn next (& mut self) -> Option < Self :: Item > { match self . inner . next () { Some ((_ , flag)) => Some (flag) , None if ! self . done => { self . done = true ; if ! self . inner . remaining () . is_empty () { Some (B :: from_bits_retain (self . inner . remaining . bits ())) } else { None } } None => None , } } }
};
}
