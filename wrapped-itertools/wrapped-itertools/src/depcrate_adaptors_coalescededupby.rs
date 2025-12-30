// Generated macro for DedupBy (type)
macro_rules! Depcrate_adaptors_coalesceDedupBy {
() => {
// Module: crate::adaptors::coalesce
// Provides: {"DedupBy"}
// Dependencies: {}
# [doc = " An iterator adaptor that removes repeated duplicates, determining equality using a comparison function."] # [doc = ""] # [doc = " See [`.dedup_by()`](crate::Itertools::dedup_by) or [`.dedup()`](crate::Itertools::dedup) for more information."] pub type DedupBy < I , Pred > = CoalesceBy < I , DedupPred2CoalescePred < Pred > , NoCount > ;
};
}
