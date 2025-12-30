// Generated macro for Sender (trait)
macro_rules! Depcrate_channelSender {
() => {
// Module: crate::channel
// Provides: {"Sender"}
// Dependencies: {}
# [doc = " Abstraction over different connections that send data"] pub trait Sender { # [doc = " Schedules a message for sending."] # [doc = ""] # [doc = " Returns a serial number than can be used to match against a reply."] fn send (& self , msg : Message) -> Result < u32 , () > ; }
};
}
