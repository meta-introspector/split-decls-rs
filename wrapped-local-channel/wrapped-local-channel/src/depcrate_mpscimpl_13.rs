// Generated macro for impl_13 (impl)
macro_rules! Depcrate_mpscimpl_13 {
() => {
// Module: crate::mpsc
// Provides: {"impl_13"}
// Dependencies: {}
impl < T > Sender < T > { # [doc = " Sends the provided message along this channel."] pub fn send (& self , item : T) -> Result < () , SendError < T > > { let mut shared = self . shared . borrow_mut () ; if ! shared . has_receiver { return Err (SendError (item)) ; } ; shared . buffer . push_back (item) ; shared . blocked_recv . wake () ; Ok (()) } # [doc = " Closes the sender half."] # [doc = ""] # [doc = " This prevents any further messages from being sent on the channel, by any sender, while"] # [doc = " still enabling the receiver to drain messages that are already buffered."] pub fn close (& mut self) { self . shared . borrow_mut () . has_receiver = false ; } }
};
}
