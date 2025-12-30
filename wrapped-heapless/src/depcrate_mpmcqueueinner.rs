// Generated macro for QueueInner (struct)
macro_rules! Depcrate_mpmcQueueInner {
() => {
// Module: crate::mpmc
// Provides: {"QueueInner"}
// Dependencies: {}
# [doc = " Base struct for [`Queue`] and [`QueueView`], generic over the [`Storage`]."] # [doc = ""] # [doc = " In most cases you should use [`Queue`] or [`QueueView`] directly. Only use this"] # [doc = " struct if you want to write code that's generic over both."] pub struct QueueInner < T , S : Storage > { dequeue_pos : AtomicTargetSize , enqueue_pos : AtomicTargetSize , buffer : UnsafeCell < S :: Buffer < Cell < T > > > , }
};
}
