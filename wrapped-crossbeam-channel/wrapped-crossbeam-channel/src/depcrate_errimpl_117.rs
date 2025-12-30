// Generated macro for impl_117 (impl)
macro_rules! Depcrate_errimpl_117 {
() => {
// Module: crate::err
// Provides: {"impl_117"}
// Dependencies: {}
impl TryRecvError { # [doc = " Returns `true` if the receive operation failed because the channel is empty."] pub fn is_empty (& self) -> bool { matches ! (self , Self :: Empty) } # [doc = " Returns `true` if the receive operation failed because the channel is disconnected."] pub fn is_disconnected (& self) -> bool { matches ! (self , Self :: Disconnected) } }
};
}
