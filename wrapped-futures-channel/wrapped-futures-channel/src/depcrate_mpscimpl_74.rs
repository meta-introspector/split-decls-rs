// Generated macro for impl_74 (impl)
macro_rules! Depcrate_mpscimpl_74 {
() => {
// Module: crate::mpsc
// Provides: {"impl_74"}
// Dependencies: {}
impl SendError { # [doc = " Returns `true` if this error is a result of the channel being full."] pub fn is_full (& self) -> bool { matches ! (self . kind , SendErrorKind :: Full) } # [doc = " Returns `true` if this error is a result of the receiver being dropped."] pub fn is_disconnected (& self) -> bool { matches ! (self . kind , SendErrorKind :: Disconnected) } }
};
}
