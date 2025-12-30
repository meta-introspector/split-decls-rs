// Generated macro for range_overlaps (function)
macro_rules! Depcrate_rangesrange_overlaps {
() => {
// Module: crate::ranges
// Provides: {"range_overlaps"}
// Dependencies: {}
fn range_overlaps (r : & Range < u64 > , other : & Range < u64 >) -> bool { other . start >= r . start && other . start <= r . end || other . end >= r . start && other . end <= r . end }
};
}
