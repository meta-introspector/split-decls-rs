// Generated macro for ReadCtrl (struct)
macro_rules! Depcrate_drivers_virtio_virtqueue_packedReadCtrl {
() => {
// Module: crate::drivers::virtio::virtqueue::packed
// Provides: {"ReadCtrl"}
// Dependencies: {}
struct ReadCtrl < 'a > { # [doc = " Poll index of the ring at init of ReadCtrl"] position : u16 , modulo : u16 , desc_ring : & 'a mut DescriptorRing , }
};
}
