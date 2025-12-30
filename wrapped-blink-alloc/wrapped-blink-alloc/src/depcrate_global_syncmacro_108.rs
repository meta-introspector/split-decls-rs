// Generated macro for macro_108 (macro)
macro_rules! Depcrate_global_syncmacro_108 {
() => {
// Module: crate::global::sync
// Provides: {"macro_108"}
// Dependencies: {}
switch_std_default ! { # [doc = " [`GlobalAlloc`] implementation based on [`SyncBlinkAlloc`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use blink_alloc::GlobalBlinkAlloc;"] # [doc = ""] # [doc = " #[global_allocator]"] # [doc = " static GLOBAL_ALLOC: GlobalBlinkAlloc = GlobalBlinkAlloc::new();"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let _ = Box::new(42);"] # [doc = "     let _ = vec![1, 2, 3];"] # [doc = " }"] # [doc = " ```"] pub struct GlobalBlinkAlloc < A : Allocator = + std :: alloc :: System > { state : UnsafeCell < State < A >>, # [cfg (debug_assertions)] allocations : AtomicU64 , } }
};
}
