// Generated macro for impl_403 (impl)
macro_rules! Depcrate_util_intimpl_403 {
() => {
// Module: crate::util::int
// Provides: {"impl_403"}
// Dependencies: {}
impl I64 for i64 { fn as_usize (self) -> usize { # [cfg (debug_assertions)] { usize :: try_from (self) . expect ("i64 overflowed usize") } # [cfg (not (debug_assertions))] { self as usize } } fn to_bits (self) -> u64 { self as u64 } fn from_bits (n : u64) -> i64 { n as i64 } }
};
}
