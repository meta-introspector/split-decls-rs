// Generated macro for impl_176 (impl)
macro_rules! Depcrate_hir_intervalimpl_176 {
() => {
// Module: crate::hir::interval
// Provides: {"impl_176"}
// Dependencies: {}
impl Bound for u8 { fn min_value () -> Self { u8 :: MIN } fn max_value () -> Self { u8 :: MAX } fn as_u32 (self) -> u32 { u32 :: from (self) } fn increment (self) -> Self { self . checked_add (1) . unwrap () } fn decrement (self) -> Self { self . checked_sub (1) . unwrap () } }
};
}
