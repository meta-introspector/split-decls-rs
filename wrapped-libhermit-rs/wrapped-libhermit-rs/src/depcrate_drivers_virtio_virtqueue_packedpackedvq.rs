// Generated macro for PackedVq (struct)
macro_rules! Depcrate_drivers_virtio_virtqueue_packedPackedVq {
() => {
// Module: crate::drivers::virtio::virtqueue::packed
// Provides: {"PackedVq"}
// Dependencies: {}
# [doc = " Packed virtqueue which provides the functionilaty as described in the"] # [doc = " virtio specification v1.1. - 2.7"] pub struct PackedVq { # [doc = " Ring which allows easy access to the raw ring structure of the"] # [doc = " specification"] descr_ring : DescriptorRing , # [doc = " Allows to tell the device if notifications are wanted"] drv_event : DrvNotif , # [doc = " Allows to check, if the device wants a notification"] dev_event : DevNotif , # [doc = " Actually notify device about avail buffers"] notif_ctrl : NotifCtrl , # [doc = " The size of the queue, equals the number of descriptors which can"] # [doc = " be used"] size : u16 , # [doc = " The virtqueues index. This identifies the virtqueue to the"] # [doc = " device and is unique on a per device basis."] index : u16 , last_next : Cell < RingIdx > , }
};
}
