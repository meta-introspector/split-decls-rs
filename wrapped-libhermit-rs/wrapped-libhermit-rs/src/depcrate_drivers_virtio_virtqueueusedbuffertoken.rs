// Generated macro for UsedBufferToken (struct)
macro_rules! Depcrate_drivers_virtio_virtqueueUsedBufferToken {
() => {
// Module: crate::drivers::virtio::virtqueue
// Provides: {"UsedBufferToken"}
// Dependencies: {}
pub (crate) struct UsedBufferToken { pub send_buff : SmallVec < [BufferElem ; 2] > , pub used_recv_buff : UsedDeviceWritableBuffer , }
};
}
