// Generated macro for impl_418 (impl)
macro_rules! Depcrate_commitimpl_418 {
() => {
// Module: crate::commit
// Provides: {"impl_418"}
// Dependencies: {}
# [doc = " Aborts iteration when a commit cannot be found"] impl < 'repo , 'commit > DoubleEndedIterator for Parents < 'commit , 'repo > { fn next_back (& mut self) -> Option < Commit < 'repo > > { self . range . next_back () . and_then (| i | self . commit . parent (i) . ok ()) } }
};
}
