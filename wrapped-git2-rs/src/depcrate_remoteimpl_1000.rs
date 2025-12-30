// Generated macro for impl_1000 (impl)
macro_rules! Depcrate_remoteimpl_1000 {
() => {
// Module: crate::remote
// Provides: {"impl_1000"}
// Dependencies: {}
impl < 'repo > Iterator for Refspecs < 'repo > { type Item = Refspec < 'repo > ; fn next (& mut self) -> Option < Refspec < 'repo > > { self . range . next () . and_then (| i | self . remote . get_refspec (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
};
}
