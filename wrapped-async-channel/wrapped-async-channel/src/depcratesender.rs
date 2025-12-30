// Generated macro for Sender (struct)
macro_rules! DepcrateSender {
() => {
// Module: crate
// Provides: {"Sender"}
// Dependencies: {}
# [doc = " The sending side of a channel."] # [doc = ""] # [doc = " Senders can be cloned and shared among threads. When all senders associated with a channel are"] # [doc = " dropped, the channel becomes closed."] # [doc = ""] # [doc = " The channel can also be closed manually by calling [`Sender::close()`]."] pub struct Sender < T > { # [doc = " Inner channel state."] channel : Arc < Channel < T > > , }
};
}
