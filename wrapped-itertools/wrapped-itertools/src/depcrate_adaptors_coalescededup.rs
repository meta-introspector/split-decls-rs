// Generated macro for Dedup (type)
macro_rules! Depcrate_adaptors_coalesceDedup {
() => {
// Module: crate::adaptors::coalesce
// Provides: {"Dedup"}
// Dependencies: {}
# [doc = " An iterator adaptor that removes repeated duplicates."] # [doc = ""] # [doc = " See [`.dedup()`](crate::Itertools::dedup) for more information."] pub type Dedup < I > = DedupBy < I , DedupEq > ;
};
}
