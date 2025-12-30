// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl < T > TrySendError < T > { # [doc = " Unwraps the message that couldn't be sent."] pub fn into_inner (self) -> T { match self { TrySendError :: Full (t) => t , TrySendError :: Closed (t) => t , } } # [doc = " Returns `true` if the channel is full but not closed."] pub fn is_full (& self) -> bool { match self { TrySendError :: Full (_) => true , TrySendError :: Closed (_) => false , } } # [doc = " Returns `true` if the channel is closed."] pub fn is_closed (& self) -> bool { match self { TrySendError :: Full (_) => false , TrySendError :: Closed (_) => true , } } }
};
}
