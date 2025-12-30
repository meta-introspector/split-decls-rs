// Generated macro for impl_634 (impl)
macro_rules! Depcrate_drivers_virtio_virtqueueimpl_634 {
() => {
// Module: crate::drivers::virtio::virtqueue
// Provides: {"impl_634"}
// Dependencies: {}
impl VirtqDescriptor for virtq :: Desc { fn flags_mut (& mut self) -> & mut virtq :: DescF { & mut self . flags } fn incomplete_desc (addr : le64 , len : le32 , flags : virtq :: DescF) -> Self { Self { addr , len , flags , next : 0 . into () , } } }
};
}
