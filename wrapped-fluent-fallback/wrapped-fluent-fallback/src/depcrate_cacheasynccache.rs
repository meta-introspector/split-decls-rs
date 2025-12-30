// Generated macro for AsyncCache (struct)
macro_rules! Depcrate_cacheAsyncCache {
() => {
// Module: crate::cache
// Provides: {"AsyncCache"}
// Dependencies: {}
pub struct AsyncCache < S , R > where S : Stream , { stream : PinCell < S > , items : UnsafeCell < ChunkyVec < S :: Item > > , pending_wakes : RefCell < Vec < Waker > > , res : std :: marker :: PhantomData < R > , }
};
}
