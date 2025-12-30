// Generated macro for Inner (struct)
macro_rules! Depcrate_cacheInner {
() => {
// Module: crate::cache
// Provides: {"Inner"}
// Dependencies: {}
struct Inner < A : Allocator > { # [doc = " Array of [`BlinkAlloc`] instances ready to pop."] pop_array : Vec < UnsafeCell < ManuallyDrop < BlinkAlloc < A > > > > , # [doc = " Index of the next pop [`BlinkAlloc`] instance to pop."] next_pop : AtomicUsize , # [doc = " Array of [`BlinkAlloc`] instances that are released."] push_array : Vec < UnsafeCell < MaybeUninit < BlinkAlloc < A > > > > , # [doc = " Index to push next [`BlinkAlloc`] instance."] next_push : AtomicUsize , }
};
}
