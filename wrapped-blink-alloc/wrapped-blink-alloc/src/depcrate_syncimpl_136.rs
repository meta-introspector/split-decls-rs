// Generated macro for impl_136 (impl)
macro_rules! Depcrate_syncimpl_136 {
() => {
// Module: crate::sync
// Provides: {"impl_136"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl SyncBlinkAlloc < Global > { # [doc = " Creates new blink allocator that uses global allocator"] # [doc = " to allocate memory chunks."] # [doc = ""] # [doc = " See [`SyncBlinkAlloc::new_in`] for using custom allocator."] # [inline (always)] pub const fn new () -> Self { SyncBlinkAlloc :: new_in (Global) } }
};
}
