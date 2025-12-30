// Generated macro for impl_181 (impl)
macro_rules! Depcrate_channelimpl_181 {
() => {
// Module: crate::channel
// Provides: {"impl_181"}
// Dependencies: {}
impl < D , E > Channel < D , E > { # [doc = " Create a new channel body."] # [doc = ""] # [doc = " The channel will buffer up to the provided number of messages. Once the buffer is full,"] # [doc = " attempts to send new messages will wait until a message is received from the channel. The"] # [doc = " provided buffer capacity must be at least 1."] pub fn new (buffer : usize) -> (Sender < D , E > , Self) { let (tx_frame , rx_frame) = mpsc :: channel (buffer) ; let (tx_error , rx_error) = oneshot :: channel () ; (Sender { tx_frame , tx_error } , Self { rx_frame , rx_error }) } }
};
}
