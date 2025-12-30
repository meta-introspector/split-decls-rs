// Generated macro for impl_60 (impl)
macro_rules! Depcrate_commit_ref_iterimpl_60 {
() => {
// Module: crate::commit::ref_iter
// Provides: {"impl_60"}
// Dependencies: {}
# [doc = " Lifecycle"] impl < 'a > CommitRefIter < 'a > { # [doc = " Create a commit iterator from data."] pub fn from_bytes (data : & 'a [u8]) -> CommitRefIter < 'a > { CommitRefIter { data , state : State :: default () , } } }
};
}
