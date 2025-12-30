// Generated macro for impl_1029 (impl)
macro_rules! Depcrate_util_intimpl_1029 {
() => {
// Module: crate::util::int
// Provides: {"impl_1029"}
// Dependencies: {}
impl Usize for usize { fn as_u8 (self) -> u8 { # [cfg (debug_assertions)] { u8 :: try_from (self) . expect ("usize overflowed u8") } # [cfg (not (debug_assertions))] { self as u8 } } fn as_u16 (self) -> u16 { # [cfg (debug_assertions)] { u16 :: try_from (self) . expect ("usize overflowed u16") } # [cfg (not (debug_assertions))] { self as u16 } } fn as_u32 (self) -> u32 { # [cfg (debug_assertions)] { u32 :: try_from (self) . expect ("usize overflowed u32") } # [cfg (not (debug_assertions))] { self as u32 } } fn as_u64 (self) -> u64 { # [cfg (debug_assertions)] { u64 :: try_from (self) . expect ("usize overflowed u64") } # [cfg (not (debug_assertions))] { self as u64 } } }
};
}
