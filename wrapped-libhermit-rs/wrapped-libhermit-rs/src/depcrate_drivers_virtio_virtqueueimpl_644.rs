// Generated macro for impl_644 (impl)
macro_rules! Depcrate_drivers_virtio_virtqueueimpl_644 {
() => {
// Module: crate::drivers::virtio::virtqueue
// Provides: {"impl_644"}
// Dependencies: {}
impl UsedBufferToken { fn from_avail_buffer_token (tkn : AvailBufferToken , written_len : u32) -> Self { Self { send_buff : tkn . send_buff , used_recv_buff : UsedDeviceWritableBuffer { elems : tkn . recv_buff , remaining_written_len : written_len , } , } } }
};
}
