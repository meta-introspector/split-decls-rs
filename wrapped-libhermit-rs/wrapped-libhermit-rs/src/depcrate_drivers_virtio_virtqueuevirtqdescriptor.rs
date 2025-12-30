// Generated macro for VirtqDescriptor (trait)
macro_rules! Depcrate_drivers_virtio_virtqueueVirtqDescriptor {
() => {
// Module: crate::drivers::virtio::virtqueue
// Provides: {"VirtqDescriptor"}
// Dependencies: {}
trait VirtqDescriptor { fn flags_mut (& mut self) -> & mut virtq :: DescF ; fn incomplete_desc (addr : virtio :: le64 , len : virtio :: le32 , flags : virtq :: DescF) -> Self ; }
};
}
