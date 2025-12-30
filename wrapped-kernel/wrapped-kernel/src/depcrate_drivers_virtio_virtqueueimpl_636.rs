// Generated macro for impl_636 (impl)
macro_rules! Depcrate_drivers_virtio_virtqueueimpl_636 {
() => {
// Module: crate::drivers::virtio::virtqueue
// Provides: {"impl_636"}
// Dependencies: {}
impl VirtqDescriptor for pvirtq :: Desc { fn flags_mut (& mut self) -> & mut virtq :: DescF { & mut self . flags } fn incomplete_desc (addr : le64 , len : le32 , flags : virtq :: DescF) -> Self { Self { addr , len , flags , id : 0 . into () , } } }
};
}
