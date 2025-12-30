// Generated macro for impl_16 (impl)
macro_rules! Depcrate_joinimpl_16 {
() => {
// Module: crate::join
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'me , Tuple : Ord > JoinInput < 'me , Tuple > for & 'me Relation < Tuple > { type RecentTuples = & 'me [Tuple] ; type StableTuples = & 'me [Relation < Tuple >] ; fn recent (self) -> Self :: RecentTuples { & [] } fn stable (self) -> Self :: StableTuples { std :: slice :: from_ref (self) } }
};
}
