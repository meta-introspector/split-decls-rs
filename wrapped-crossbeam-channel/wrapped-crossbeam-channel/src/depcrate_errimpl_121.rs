// Generated macro for impl_121 (impl)
macro_rules! Depcrate_errimpl_121 {
() => {
// Module: crate::err
// Provides: {"impl_121"}
// Dependencies: {}
impl RecvTimeoutError { # [doc = " Returns `true` if the receive operation timed out."] pub fn is_timeout (& self) -> bool { matches ! (self , Self :: Timeout) } # [doc = " Returns `true` if the receive operation failed because the channel is disconnected."] pub fn is_disconnected (& self) -> bool { matches ! (self , Self :: Disconnected) } }
};
}
