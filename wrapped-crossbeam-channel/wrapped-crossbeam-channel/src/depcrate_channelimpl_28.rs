// Generated macro for impl_28 (impl)
macro_rules! Depcrate_channelimpl_28 {
() => {
// Module: crate::channel
// Provides: {"impl_28"}
// Dependencies: {}
impl < T > Clone for Sender < T > { fn clone (& self) -> Self { let flavor = match & self . flavor { SenderFlavor :: Array (chan) => SenderFlavor :: Array (chan . acquire ()) , SenderFlavor :: List (chan) => SenderFlavor :: List (chan . acquire ()) , SenderFlavor :: Zero (chan) => SenderFlavor :: Zero (chan . acquire ()) , } ; Self { flavor } } }
};
}
