// Generated macro for WriteCtrl (struct)
macro_rules! Depcrate_drivers_virtio_virtqueue_packedWriteCtrl {
() => {
// Module: crate::drivers::virtio::virtqueue::packed
// Provides: {"WriteCtrl"}
// Dependencies: {}
# [doc = " Convenient struct that allows to conveniently write descriptors into the queue."] # [doc = " The struct takes care of updating the state of the queue correctly and to write"] # [doc = " the correct flags."] struct WriteCtrl < 'a > { # [doc = " Where did the write of the buffer start in the descriptor ring"] # [doc = " This is important, as we must make this descriptor available"] # [doc = " lastly."] start : u16 , # [doc = " Where to write next. This should always be equal to the Rings"] # [doc = " write_next field."] position : u16 , modulo : u16 , # [doc = " The [pvirtq::Desc::flags] value for the first descriptor, the write of which is deferred."] first_flags : DescF , # [doc = " Buff ID of this write"] buff_id : u16 , desc_ring : & 'a mut DescriptorRing , }
};
}
