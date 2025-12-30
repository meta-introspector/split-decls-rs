// Generated macro for impl_421 (impl)
macro_rules! Depcrate_commitimpl_421 {
() => {
// Module: crate::commit
// Provides: {"impl_421"}
// Dependencies: {}
# [doc = " Aborts iteration when a commit cannot be found"] impl < 'commit > Iterator for ParentIds < 'commit > { type Item = Oid ; fn next (& mut self) -> Option < Oid > { self . range . next () . and_then (| i | self . commit . parent_id (i) . ok ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
};
}
