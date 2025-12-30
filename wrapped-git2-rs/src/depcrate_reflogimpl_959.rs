// Generated macro for impl_959 (impl)
macro_rules! Depcrate_reflogimpl_959 {
() => {
// Module: crate::reflog
// Provides: {"impl_959"}
// Dependencies: {}
impl < 'reflog > Iterator for ReflogIter < 'reflog > { type Item = ReflogEntry < 'reflog > ; fn next (& mut self) -> Option < ReflogEntry < 'reflog > > { self . range . next () . and_then (| i | self . reflog . get (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
};
}
