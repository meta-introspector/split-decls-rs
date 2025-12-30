// Generated macro for impl_64 (impl)
macro_rules! Depcrate_blinkimpl_64 {
() => {
// Module: crate::blink
// Provides: {"impl_64"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Blink < BlinkAlloc < Global > > { # [doc = " Creates new blink instance with `BlinkAlloc` baked by `Global`"] # [doc = " allocator."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use blink_alloc::Blink;"] # [doc = " let mut blink = Blink::new();"] # [doc = ""] # [doc = " blink.put(42);"] # [doc = " ```"] # [inline (always)] pub const fn new () -> Self { Blink :: new_in (BlinkAlloc :: new ()) } # [doc = " Creates new blink instance with `BlinkAlloc` baked by `Global`"] # [doc = " allocator."] # [doc = " `BlinkAlloc` receives starting chunk size."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use blink_alloc::Blink;"] # [doc = " let mut blink = Blink::with_chunk_size(16);"] # [doc = ""] # [doc = " blink.put(42);"] # [inline (always)] pub const fn with_chunk_size (capacity : usize) -> Self { Blink :: new_in (BlinkAlloc :: with_chunk_size (capacity)) } }
};
}
