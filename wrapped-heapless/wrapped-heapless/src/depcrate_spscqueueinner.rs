// Generated macro for QueueInner (struct)
macro_rules! Depcrate_spscQueueInner {
() => {
// Module: crate::spsc
// Provides: {"QueueInner"}
// Dependencies: {}
# [doc = " Base struct for [`Queue`] and [`QueueView`], generic over the [`Storage`]."] # [doc = ""] # [doc = " In most cases you should use [`Queue`] or [`QueueView`] directly. Only use this"] # [doc = " struct if you want to write code that's generic over both."] pub struct QueueInner < T , S : Storage > { pub (crate) head : AtomicUsize , pub (crate) tail : AtomicUsize , pub (crate) buffer : S :: Buffer < UnsafeCell < MaybeUninit < T > > > , }
};
}
