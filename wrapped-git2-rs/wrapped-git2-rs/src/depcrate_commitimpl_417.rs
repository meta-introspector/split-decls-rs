// Generated macro for impl_417 (impl)
macro_rules! Depcrate_commitimpl_417 {
() => {
// Module: crate::commit
// Provides: {"impl_417"}
// Dependencies: {}
# [doc = " Aborts iteration when a commit cannot be found"] impl < 'repo , 'commit > Iterator for Parents < 'commit , 'repo > { type Item = Commit < 'repo > ; fn next (& mut self) -> Option < Commit < 'repo > > { self . range . next () . and_then (| i | self . commit . parent (i) . ok ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
};
}
