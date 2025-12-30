// Generated macro for impl_5 (impl)
macro_rules! Depcrate_errorimpl_5 {
() => {
// Module: crate::error
// Provides: {"impl_5"}
// Dependencies: {}
impl ProtocolError { pub (crate) fn new (msg : impl Into < String >) -> Self { ProtocolError (msg . into () , false) } pub (crate) fn disconnected () -> ProtocolError { ProtocolError ("disconnected channel" . into () , true) } # [doc = " Whether this error occurred due to a disconnected channel."] pub fn channel_is_disconnected (& self) -> bool { self . 1 } }
};
}
