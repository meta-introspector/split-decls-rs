// Generated macro for impl_399 (impl)
macro_rules! Depcrate_util_intimpl_399 {
() => {
// Module: crate::util::int
// Provides: {"impl_399"}
// Dependencies: {}
impl I8 for i8 { fn as_usize (self) -> usize { # [cfg (debug_assertions)] { usize :: try_from (self) . expect ("i8 overflowed usize") } # [cfg (not (debug_assertions))] { self as usize } } fn to_bits (self) -> u8 { self as u8 } fn from_bits (n : u8) -> i8 { n as i8 } }
};
}
