// Generated macro for IterPinRef (struct)
macro_rules! Depcrate_stream_futures_unordered_iterIterPinRef {
() => {
// Module: crate::stream::futures_unordered::iter
// Provides: {"IterPinRef"}
// Dependencies: {}
# [doc = " Immutable iterator over all futures in the unordered set."] # [derive (Debug)] pub struct IterPinRef < 'a , Fut > { pub (super) task : * const Task < Fut > , pub (super) len : usize , pub (super) pending_next_all : * mut Task < Fut > , pub (super) _marker : PhantomData < & 'a FuturesUnordered < Fut > > , }
};
}
