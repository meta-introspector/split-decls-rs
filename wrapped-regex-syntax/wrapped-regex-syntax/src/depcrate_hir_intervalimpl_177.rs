// Generated macro for impl_177 (impl)
macro_rules! Depcrate_hir_intervalimpl_177 {
() => {
// Module: crate::hir::interval
// Provides: {"impl_177"}
// Dependencies: {}
impl Bound for char { fn min_value () -> Self { '\x00' } fn max_value () -> Self { '\u{10FFFF}' } fn as_u32 (self) -> u32 { u32 :: from (self) } fn increment (self) -> Self { match self { '\u{D7FF}' => '\u{E000}' , c => char :: from_u32 (u32 :: from (c) . checked_add (1) . unwrap ()) . unwrap () , } } fn decrement (self) -> Self { match self { '\u{E000}' => '\u{D7FF}' , c => char :: from_u32 (u32 :: from (c) . checked_sub (1) . unwrap ()) . unwrap () , } } }
};
}
