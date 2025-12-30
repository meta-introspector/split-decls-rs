// Generated macro for Inner (struct)
macro_rules! Depcrate_address_channelInner {
() => {
// Module: crate::address::channel
// Provides: {"Inner"}
// Dependencies: {}
struct Inner < A : Actor > { buffer : AtomicUsize , state : AtomicUsize , message_queue : Queue < Envelope < A > > , parked_queue : Queue < Arc < Mutex < SenderTask > > > , num_senders : AtomicUsize , recv_task : AtomicWaker , }
};
}
