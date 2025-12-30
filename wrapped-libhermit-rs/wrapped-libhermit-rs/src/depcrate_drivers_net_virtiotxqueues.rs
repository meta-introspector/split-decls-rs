// Generated macro for TxQueues (struct)
macro_rules! Depcrate_drivers_net_virtioTxQueues {
() => {
// Module: crate::drivers::net::virtio
// Provides: {"TxQueues"}
// Dependencies: {}
# [doc = " Structure which handles transmission of packets and delegation"] # [doc = " to the respective queue structures."] pub struct TxQueues { vqs : Vec < VirtQueue > , buf_size : u32 , }
};
}
