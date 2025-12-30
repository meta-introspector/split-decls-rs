// Generated macro for impl_616 (impl)
macro_rules! Depcrate_drivers_virtio_virtqueue_splitimpl_616 {
() => {
// Module: crate::drivers::virtio::virtqueue::split
// Provides: {"impl_616"}
// Dependencies: {}
impl VirtqPrivate for SplitVq { type Descriptor = virtq :: Desc ; fn create_indirect_ctrl (buffer_tkn : & AvailBufferToken ,) -> Result < Box < [Self :: Descriptor] > , VirtqError > { Ok (Self :: descriptor_iter (buffer_tkn) ? . zip (1 ..) . map (| (descriptor , next_id) | Self :: Descriptor { next : next_id . into () , .. descriptor }) . collect :: < Vec < _ > > () . into_boxed_slice ()) } }
};
}
