// Generated macro for DedupByWithCount (type)
macro_rules! Depcrate_adaptors_coalesceDedupByWithCount {
() => {
// Module: crate::adaptors::coalesce
// Provides: {"DedupByWithCount"}
// Dependencies: {}
# [doc = " An iterator adaptor that removes repeated duplicates, while keeping a count of how many"] # [doc = " repeated elements were present. This will determine equality using a comparison function."] # [doc = ""] # [doc = " See [`.dedup_by_with_count()`](crate::Itertools::dedup_by_with_count) or"] # [doc = " [`.dedup_with_count()`](crate::Itertools::dedup_with_count) for more information."] pub type DedupByWithCount < I , Pred > = CoalesceBy < I , DedupPredWithCount2CoalescePred < Pred > , WithCount > ;
};
}
