// Generated macro for Enumerate (struct)
macro_rules! Depcrate_concurrent_stream_enumerateEnumerate {
() => {
// Module: crate::concurrent_stream::enumerate
// Provides: {"Enumerate"}
// Dependencies: {}
# [doc = " A concurrent iterator that yields the current count and the element during iteration."] # [doc = ""] # [doc = " This `struct` is created by the [`enumerate`] method on [`ConcurrentStream`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`enumerate`]: ConcurrentStream::enumerate"] # [doc = " [`ConcurrentStream`]: trait.ConcurrentStream.html"] # [derive (Debug)] pub struct Enumerate < CS : ConcurrentStream > { inner : CS , }
};
}
