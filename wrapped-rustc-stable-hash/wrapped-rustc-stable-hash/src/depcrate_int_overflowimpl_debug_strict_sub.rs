// Generated macro for impl_debug_strict_sub (macro)
macro_rules! Depcrate_int_overflowimpl_debug_strict_sub {
() => {
// Module: crate::int_overflow
// Provides: {"impl_debug_strict_sub"}
// Dependencies: {}
macro_rules ! impl_debug_strict_sub { ($ ($ ty : ty) *) => { $ (impl DebugStrictSub for $ ty { fn debug_strict_sub (self , other : Self) -> Self { if cfg ! (debug_assertions) { self - other } else { self . wrapping_sub (other) } } }) * } ; }
};
}
