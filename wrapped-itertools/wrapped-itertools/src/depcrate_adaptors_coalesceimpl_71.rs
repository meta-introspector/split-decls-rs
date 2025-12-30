// Generated macro for impl_71 (impl)
macro_rules! Depcrate_adaptors_coalesceimpl_71 {
() => {
// Module: crate::adaptors::coalesce
// Provides: {"impl_71"}
// Dependencies: {}
impl < DP , T > CoalescePredicate < T , (usize , T) > for DedupPredWithCount2CoalescePred < DP > where DP : DedupPredicate < T > , { fn coalesce_pair (& mut self , (c , t) : (usize , T) , item : T ,) -> Result < (usize , T) , ((usize , T) , (usize , T)) > { if self . 0 . dedup_pair (& t , & item) { Ok ((c + 1 , t)) } else { Err (((c , t) , (1 , item))) } } }
};
}
