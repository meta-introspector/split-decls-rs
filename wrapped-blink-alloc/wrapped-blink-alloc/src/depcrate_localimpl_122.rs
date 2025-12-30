// Generated macro for impl_122 (impl)
macro_rules! Depcrate_localimpl_122 {
() => {
// Module: crate::local
// Provides: {"impl_122"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl BlinkAlloc < Global > { # [doc = " Creates new blink allocator that uses global allocator"] # [doc = " to allocate memory chunks."] # [doc = ""] # [doc = " See [`BlinkAlloc::new_in`] for using custom allocator."] # [inline] pub const fn new () -> Self { BlinkAlloc :: new_in (Global) } # [doc = " Creates new blink allocator that uses global allocator"] # [doc = " to allocate memory chunks."] # [doc = " With this method you can specify initial chunk size."] # [doc = ""] # [doc = " See [`BlinkAlloc::new_in`] for using custom allocator."] # [inline] pub const fn with_chunk_size (chunk_size : usize) -> Self { BlinkAlloc :: with_chunk_size_in (chunk_size , Global) } }
};
}
