// Generated macro for AvailBufferToken (struct)
macro_rules! Depcrate_drivers_virtio_virtqueueAvailBufferToken {
() => {
// Module: crate::drivers::virtio::virtqueue
// Provides: {"AvailBufferToken"}
// Dependencies: {}
# [doc = " The struct represents buffers which are ready to be written or to be send."] # [doc = ""] # [doc = " BufferTokens can be written in two ways:"] # [doc = " * in one step via `BufferToken.write()"] # [doc = "   * consumes BufferToken and returns a TransferToken"] # [doc = " * sequentially via `BufferToken.write_seq()"] # [doc = ""] # [doc = " # Structure of the Token"] # [doc = " The token can potentially hold both a *send* and a *recv* buffer, but MUST hold"] # [doc = " one."] # [doc = " The *send* buffer is the data the device will read during a transfer, the *recv* buffer"] # [doc = " is the data the device will write to during a transfer."] # [doc = ""] # [doc = " # What are Buffers"] # [doc = " A buffer represents multiple chunks of memory. Where each chunk can be of different size."] # [doc = " The chunks are named descriptors in the following."] # [doc = ""] # [doc = " **For Example:**"] # [doc = " A buffer could consist of 3 descriptors:"] # [doc = " 1. First descriptor of 30 bytes"] # [doc = " 2. Second descriptor of 10 bytes"] # [doc = " 3. Third descriptor of 100 bytes"] # [doc = ""] # [doc = " Each of these descriptors consumes one \"element\" of the"] # [doc = " respective virtqueue."] # [doc = " The maximum number of descriptors per buffer is bounded by the size of the virtqueue."] pub struct AvailBufferToken { pub (crate) send_buff : SmallVec < [BufferElem ; 2] > , pub (crate) recv_buff : SmallVec < [BufferElem ; 2] > , }
};
}
