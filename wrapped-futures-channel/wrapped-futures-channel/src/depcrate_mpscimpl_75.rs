// Generated macro for impl_75 (impl)
macro_rules! Depcrate_mpscimpl_75 {
() => {
// Module: crate::mpsc
// Provides: {"impl_75"}
// Dependencies: {}
impl TryRecvError { # [doc = " Returns `true` if the channel is empty but not closed."] pub fn is_empty (& self) -> bool { matches ! (self , TryRecvError :: Empty) } # [doc = " Returns `true` if the channel is empty and closed."] pub fn is_closed (& self) -> bool { matches ! (self , TryRecvError :: Closed) } }
};
}
