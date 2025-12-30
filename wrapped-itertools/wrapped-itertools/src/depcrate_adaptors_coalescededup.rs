// Generated macro for dedup (function)
macro_rules! Depcrate_adaptors_coalescededup {
() => {
// Module: crate::adaptors::coalesce
// Provides: {"dedup"}
// Dependencies: {}
# [doc = " Create a new `Dedup`."] pub fn dedup < I > (iter : I) -> Dedup < I > where I : Iterator , { dedup_by (iter , DedupEq) }
};
}
