// Generated macro for dedup_by_with_count (function)
macro_rules! Depcrate_adaptors_coalescededup_by_with_count {
() => {
// Module: crate::adaptors::coalesce
// Provides: {"dedup_by_with_count"}
// Dependencies: {}
# [doc = " Create a new `DedupByWithCount`."] pub fn dedup_by_with_count < I , Pred > (iter : I , dedup_pred : Pred) -> DedupByWithCount < I , Pred > where I : Iterator , { DedupByWithCount { last : None , iter , f : DedupPredWithCount2CoalescePred (dedup_pred) , } }
};
}
