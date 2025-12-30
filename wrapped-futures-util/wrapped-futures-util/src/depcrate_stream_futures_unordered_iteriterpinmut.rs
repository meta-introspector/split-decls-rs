// Generated macro for IterPinMut (struct)
macro_rules! Depcrate_stream_futures_unordered_iterIterPinMut {
() => {
// Module: crate::stream::futures_unordered::iter
// Provides: {"IterPinMut"}
// Dependencies: {}
# [doc = " Mutable iterator over all futures in the unordered set."] # [derive (Debug)] pub struct IterPinMut < 'a , Fut > { pub (super) task : * const Task < Fut > , pub (super) len : usize , pub (super) _marker : PhantomData < & 'a mut FuturesUnordered < Fut > > , }
};
}
