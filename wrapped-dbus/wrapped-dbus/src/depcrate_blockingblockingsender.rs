// Generated macro for BlockingSender (trait)
macro_rules! Depcrate_blockingBlockingSender {
() => {
// Module: crate::blocking
// Provides: {"BlockingSender"}
// Dependencies: {}
# [doc = " Abstraction over different connections"] pub trait BlockingSender { # [doc = " Sends a message over the D-Bus and blocks, waiting for a reply or a timeout. This is used for method calls."] # [doc = ""] # [doc = " Note: In case of an error reply, this is returned as an Err(), not as a Ok(Message) with the error type."] fn send_with_reply_and_block (& self , msg : Message , timeout : Duration) -> Result < Message , Error > ; }
};
}
