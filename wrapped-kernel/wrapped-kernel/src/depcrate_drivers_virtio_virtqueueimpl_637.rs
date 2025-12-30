// Generated macro for impl_637 (impl)
macro_rules! Depcrate_drivers_virtio_virtqueueimpl_637 {
() => {
// Module: crate::drivers::virtio::virtqueue
// Provides: {"impl_637"}
// Dependencies: {}
# [doc = " Public Interface for TransferToken"] impl < Descriptor > TransferToken < Descriptor > { # [doc = " Returns the number of descritprors that will be placed in the queue."] # [doc = " This number can differ from the `BufferToken.num_descr()` function value"] # [doc = " as indirect buffers only consume one descriptor in the queue, but can have"] # [doc = " more descriptors that are accessible via the descriptor in the queue."] fn num_consuming_descr (& self) -> u16 { if self . ctrl_desc . is_some () { 1 } else { self . buff_tkn . num_descr () } } }
};
}
