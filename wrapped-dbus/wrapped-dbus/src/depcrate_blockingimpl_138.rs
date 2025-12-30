// Generated macro for impl_138 (impl)
macro_rules! Depcrate_blockingimpl_138 {
() => {
// Module: crate::blocking
// Provides: {"impl_138"}
// Dependencies: {}
impl BlockingSender for Channel { fn send_with_reply_and_block (& self , msg : Message , timeout : Duration) -> Result < Message , Error > { Channel :: send_with_reply_and_block (self , msg , timeout) } }
};
}
