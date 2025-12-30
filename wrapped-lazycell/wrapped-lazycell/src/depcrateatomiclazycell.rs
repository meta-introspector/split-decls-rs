// Generated macro for AtomicLazyCell (struct)
macro_rules! DepcrateAtomicLazyCell {
() => {
// Module: crate
// Provides: {"AtomicLazyCell"}
// Dependencies: {}
# [doc = " A lazily filled and thread-safe `Cell`, with frozen contents."] # [derive (Debug)] pub struct AtomicLazyCell < T > { inner : UnsafeCell < Option < T > > , state : AtomicUsize , }
};
}
