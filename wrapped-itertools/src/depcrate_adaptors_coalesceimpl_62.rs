// Generated macro for impl_62 (impl)
macro_rules! Depcrate_adaptors_coalesceimpl_62 {
() => {
// Module: crate::adaptors::coalesce
// Provides: {"impl_62"}
// Dependencies: {}
impl < DP , T > CoalescePredicate < T , T > for DedupPred2CoalescePred < DP > where DP : DedupPredicate < T > , { fn coalesce_pair (& mut self , t : T , item : T) -> Result < T , (T , T) > { if self . 0 . dedup_pair (& t , & item) { Ok (t) } else { Err ((t , item)) } } }
};
}
