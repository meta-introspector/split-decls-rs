// Generated macro for impl_27 (impl)
macro_rules! Depcrate_channelimpl_27 {
() => {
// Module: crate::channel
// Provides: {"impl_27"}
// Dependencies: {}
impl < T > Drop for Sender < T > { fn drop (& mut self) { unsafe { match & self . flavor { SenderFlavor :: Array (chan) => chan . release (| c | c . disconnect ()) , SenderFlavor :: List (chan) => chan . release (| c | c . disconnect_senders ()) , SenderFlavor :: Zero (chan) => chan . release (| c | c . disconnect ()) , } } } }
};
}
