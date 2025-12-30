// Generated macro for impl_18 (impl)
macro_rules! Depcrate_mpscimpl_18 {
() => {
// Module: crate::mpsc
// Provides: {"impl_18"}
// Dependencies: {}
impl < T > Receiver < T > { # [doc = " Receive the next value."] # [doc = ""] # [doc = " Returns `None` if the channel is empty and has been [closed](Sender::close) explicitly or"] # [doc = " when all senders have been dropped and, therefore, no more values can ever be sent though"] # [doc = " this channel."] pub async fn recv (& mut self) -> Option < T > { let mut this = Pin :: new (self) ; poll_fn (| cx | this . as_mut () . poll_next (cx)) . await } # [doc = " Create an associated [Sender]."] pub fn sender (& self) -> Sender < T > { Sender { shared : self . shared . clone () , } } }
};
}
