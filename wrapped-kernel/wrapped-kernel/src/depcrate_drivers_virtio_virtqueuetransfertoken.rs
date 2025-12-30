// Generated macro for TransferToken (struct)
macro_rules! Depcrate_drivers_virtio_virtqueueTransferToken {
() => {
// Module: crate::drivers::virtio::virtqueue
// Provides: {"TransferToken"}
// Dependencies: {}
# [doc = " The struct represents buffers which are ready to be send via the"] # [doc = " virtqueue. Buffers can no longer be written or retrieved."] pub struct TransferToken < Descriptor > { # [doc = " Must be some in order to prevent drop"] # [doc = " upon reuse."] buff_tkn : AvailBufferToken , ctrl_desc : Option < Box < [Descriptor] > > , }
};
}
