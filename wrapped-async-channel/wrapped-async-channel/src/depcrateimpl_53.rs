// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
impl TryRecvError { # [doc = " Returns `true` if the channel is empty but not closed."] pub fn is_empty (& self) -> bool { match self { TryRecvError :: Empty => true , TryRecvError :: Closed => false , } } # [doc = " Returns `true` if the channel is empty and closed."] pub fn is_closed (& self) -> bool { match self { TryRecvError :: Empty => false , TryRecvError :: Closed => true , } } }
};
}
