// Generated macro for impl_1027 (impl)
macro_rules! Depcrate_util_intimpl_1027 {
() => {
// Module: crate::util::int
// Provides: {"impl_1027"}
// Dependencies: {}
impl I32 for i32 { fn as_usize (self) -> usize { # [cfg (debug_assertions)] { usize :: try_from (self) . expect ("i32 overflowed usize") } # [cfg (not (debug_assertions))] { self as usize } } fn to_bits (self) -> u32 { self as u32 } fn from_bits (n : u32) -> i32 { n as i32 } }
};
}
