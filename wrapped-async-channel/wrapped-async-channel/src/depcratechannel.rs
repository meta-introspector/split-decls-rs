// Generated macro for Channel (struct)
macro_rules! DepcrateChannel {
() => {
// Module: crate
// Provides: {"Channel"}
// Dependencies: {}
struct Channel < T > { # [doc = " Inner message queue."] queue : ConcurrentQueue < T > , # [doc = " Send operations waiting while the channel is full."] send_ops : Event , # [doc = " Receive operations waiting while the channel is empty and not closed."] recv_ops : Event , # [doc = " Stream operations while the channel is empty and not closed."] stream_ops : Event , # [doc = " Closed operations while the channel is not closed."] closed_ops : Event , # [doc = " The number of currently active `Sender`s."] sender_count : AtomicUsize , # [doc = " The number of currently active `Receivers`s."] receiver_count : AtomicUsize , }
};
}
