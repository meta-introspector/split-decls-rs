// Generated macro for dedup_by (function)
macro_rules! Depcrate_adaptors_coalescededup_by {
() => {
// Module: crate::adaptors::coalesce
// Provides: {"dedup_by"}
// Dependencies: {}
# [doc = " Create a new `DedupBy`."] pub fn dedup_by < I , Pred > (iter : I , dedup_pred : Pred) -> DedupBy < I , Pred > where I : Iterator , { DedupBy { last : None , iter , f : DedupPred2CoalescePred (dedup_pred) , } }
};
}
