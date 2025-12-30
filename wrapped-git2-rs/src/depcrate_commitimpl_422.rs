// Generated macro for impl_422 (impl)
macro_rules! Depcrate_commitimpl_422 {
() => {
// Module: crate::commit
// Provides: {"impl_422"}
// Dependencies: {}
# [doc = " Aborts iteration when a commit cannot be found"] impl < 'commit > DoubleEndedIterator for ParentIds < 'commit > { fn next_back (& mut self) -> Option < Oid > { self . range . next_back () . and_then (| i | self . commit . parent_id (i) . ok ()) } }
};
}
