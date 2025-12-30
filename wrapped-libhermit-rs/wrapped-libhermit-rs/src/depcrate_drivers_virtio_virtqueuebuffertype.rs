// Generated macro for BufferType (enum)
macro_rules! Depcrate_drivers_virtio_virtqueueBufferType {
() => {
// Module: crate::drivers::virtio::virtqueue
// Provides: {"BufferType"}
// Dependencies: {}
pub enum BufferType { # [doc = " As many descriptors get consumed in the descriptor table as the sum of the numbers of slices in [AvailBufferToken::send_buff] and [AvailBufferToken::recv_buff]."] Direct , # [doc = " Results in one descriptor in the queue, hence consumes one element in the main descriptor table. The queue will merge the send and recv buffers as follows:"] # [doc = " ```text"] # [doc = " //+++++++++++++++++++++++"] # [doc = " //+        Queue        +"] # [doc = " //+++++++++++++++++++++++"] # [doc = " //+ Indirect descriptor + -> refers to a descriptor list in the form of ->  ++++++++++++++++++++++++++"] # [doc = " //+         ...         +                                                   +  Descriptors for send  +"] # [doc = " //+++++++++++++++++++++++                                                   +  Descriptors for recv  +"] # [doc = " //                                                                          ++++++++++++++++++++++++++"] # [doc = " ```"] # [doc = " As a result indirect descriptors result in a single descriptor consumption in the actual queue."] Indirect , }
};
}
