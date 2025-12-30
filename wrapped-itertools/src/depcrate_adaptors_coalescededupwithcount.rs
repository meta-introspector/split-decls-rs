// Generated macro for DedupWithCount (type)
macro_rules! Depcrate_adaptors_coalesceDedupWithCount {
() => {
// Module: crate::adaptors::coalesce
// Provides: {"DedupWithCount"}
// Dependencies: {}
# [doc = " An iterator adaptor that removes repeated duplicates, while keeping a count of how many"] # [doc = " repeated elements were present."] # [doc = ""] # [doc = " See [`.dedup_with_count()`](crate::Itertools::dedup_with_count) for more information."] pub type DedupWithCount < I > = DedupByWithCount < I , DedupEq > ;
};
}
