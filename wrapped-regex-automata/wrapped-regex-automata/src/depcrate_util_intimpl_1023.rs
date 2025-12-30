// Generated macro for impl_1023 (impl)
macro_rules! Depcrate_util_intimpl_1023 {
() => {
// Module: crate::util::int
// Provides: {"impl_1023"}
// Dependencies: {}
impl U32 for u32 { fn as_usize (self) -> usize { # [cfg (debug_assertions)] { usize :: try_from (self) . expect ("u32 overflowed usize") } # [cfg (not (debug_assertions))] { self as usize } } fn low_u8 (self) -> u8 { self as u8 } fn low_u16 (self) -> u16 { self as u16 } fn high_u16 (self) -> u16 { (self >> 16) as u16 } }
};
}
