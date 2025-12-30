// Generated macro for impl_38 (impl)
macro_rules! Depcrate_channelimpl_38 {
() => {
// Module: crate::channel
// Provides: {"impl_38"}
// Dependencies: {}
impl < T > Clone for Receiver < T > { fn clone (& self) -> Self { let flavor = match & self . flavor { ReceiverFlavor :: Array (chan) => ReceiverFlavor :: Array (chan . acquire ()) , ReceiverFlavor :: List (chan) => ReceiverFlavor :: List (chan . acquire ()) , ReceiverFlavor :: Zero (chan) => ReceiverFlavor :: Zero (chan . acquire ()) , ReceiverFlavor :: At (chan) => ReceiverFlavor :: At (chan . clone ()) , ReceiverFlavor :: Tick (chan) => ReceiverFlavor :: Tick (chan . clone ()) , ReceiverFlavor :: Never (_) => ReceiverFlavor :: Never (flavors :: never :: Channel :: new ()) , } ; Self { flavor } } }
};
}
