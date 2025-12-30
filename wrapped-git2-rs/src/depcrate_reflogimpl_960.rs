// Generated macro for impl_960 (impl)
macro_rules! Depcrate_reflogimpl_960 {
() => {
// Module: crate::reflog
// Provides: {"impl_960"}
// Dependencies: {}
impl < 'reflog > DoubleEndedIterator for ReflogIter < 'reflog > { fn next_back (& mut self) -> Option < ReflogEntry < 'reflog > > { self . range . next_back () . and_then (| i | self . reflog . get (i)) } }
};
}
