// Generated macro for Inner (struct)
macro_rules! Depcrate_stream_channelInner {
() => {
// Module: crate::stream::channel
// Provides: {"Inner"}
// Dependencies: {}
struct Inner < T , E > { slot : Slot < Message < Result < T , E > > > , receiver_gone : AtomicBool , }
};
}
