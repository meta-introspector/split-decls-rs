// Generated macro for impl_37 (impl)
macro_rules! Depcrate_channelimpl_37 {
() => {
// Module: crate::channel
// Provides: {"impl_37"}
// Dependencies: {}
impl < T > Drop for Receiver < T > { fn drop (& mut self) { unsafe { match & self . flavor { ReceiverFlavor :: Array (chan) => chan . release (| c | c . disconnect ()) , ReceiverFlavor :: List (chan) => chan . release (| c | c . disconnect_receivers ()) , ReceiverFlavor :: Zero (chan) => chan . release (| c | c . disconnect ()) , ReceiverFlavor :: At (_) => { } ReceiverFlavor :: Tick (_) => { } ReceiverFlavor :: Never (_) => { } } } } }
};
}
