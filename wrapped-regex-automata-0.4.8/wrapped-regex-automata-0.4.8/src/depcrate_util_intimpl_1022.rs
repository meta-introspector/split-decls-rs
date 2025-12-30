// Generated macro for impl_1022 (impl)
macro_rules! Depcrate_util_intimpl_1022 {
() => {
// Module: crate::util::int
// Provides: {"impl_1022"}
// Dependencies: {}
impl U64 for u64 { fn as_usize (self) -> usize { # [cfg (debug_assertions)] { usize :: try_from (self) . expect ("u64 overflowed usize") } # [cfg (not (debug_assertions))] { self as usize } } fn low_u8 (self) -> u8 { self as u8 } fn low_u16 (self) -> u16 { self as u16 } fn low_u32 (self) -> u32 { self as u32 } fn high_u32 (self) -> u32 { (self >> 32) as u32 } }
};
}
