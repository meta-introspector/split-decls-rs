// Generated macro for impl_596 (impl)
macro_rules! Depcrate_drivers_virtio_virtqueue_packedimpl_596 {
() => {
// Module: crate::drivers::virtio::virtqueue::packed
// Provides: {"impl_596"}
// Dependencies: {}
impl VirtqPrivate for PackedVq { type Descriptor = pvirtq :: Desc ; fn create_indirect_ctrl (buffer_tkn : & AvailBufferToken ,) -> Result < Box < [Self :: Descriptor] > , VirtqError > { Ok (Self :: descriptor_iter (buffer_tkn) ? . collect :: < Vec < _ > > () . into_boxed_slice ()) } }
};
}
