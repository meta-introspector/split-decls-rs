// Generated macro for macro_95 (macro)
macro_rules! Depcrate_global_localmacro_95 {
() => {
// Module: crate::global::local
// Provides: {"macro_95"}
// Dependencies: {}
switch_std_default ! { # [doc = " [`GlobalAlloc`] implementation based on [`BlinkAlloc`]."] pub struct UnsafeGlobalBlinkAlloc < A : Allocator = + std :: alloc :: System > { state : UnsafeCell < State < A >>, # [cfg (debug_assertions)] allocations : Cell < u64 >, } }
};
}
