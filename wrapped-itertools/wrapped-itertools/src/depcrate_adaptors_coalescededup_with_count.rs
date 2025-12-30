// Generated macro for dedup_with_count (function)
macro_rules! Depcrate_adaptors_coalescededup_with_count {
() => {
// Module: crate::adaptors::coalesce
// Provides: {"dedup_with_count"}
// Dependencies: {}
# [doc = " Create a new `DedupWithCount`."] pub fn dedup_with_count < I > (iter : I) -> DedupWithCount < I > where I : Iterator , { dedup_by_with_count (iter , DedupEq) }
};
}
