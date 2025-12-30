// Generated macro for impl_431 (impl)
macro_rules! Depcrate_connection_pathsimpl_431 {
() => {
// Module: crate::connection::paths
// Provides: {"impl_431"}
// Dependencies: {}
impl InFlight { fn new () -> Self { Self { bytes : 0 , ack_eliciting : 0 , } } fn insert (& mut self , packet : & SentPacket) { self . bytes += u64 :: from (packet . size) ; self . ack_eliciting += u64 :: from (packet . ack_eliciting) ; } # [doc = " Update counters to account for a packet becoming acknowledged, lost, or abandoned"] fn remove (& mut self , packet : & SentPacket) { self . bytes -= u64 :: from (packet . size) ; self . ack_eliciting -= u64 :: from (packet . ack_eliciting) ; } }
};
}
