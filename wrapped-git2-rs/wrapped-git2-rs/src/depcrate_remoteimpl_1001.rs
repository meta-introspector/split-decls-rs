// Generated macro for impl_1001 (impl)
macro_rules! Depcrate_remoteimpl_1001 {
() => {
// Module: crate::remote
// Provides: {"impl_1001"}
// Dependencies: {}
impl < 'repo > DoubleEndedIterator for Refspecs < 'repo > { fn next_back (& mut self) -> Option < Refspec < 'repo > > { self . range . next_back () . and_then (| i | self . remote . get_refspec (i)) } }
};
}
