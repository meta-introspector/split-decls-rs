// Generated macro for impl_106 (impl)
macro_rules! Depcrate_errimpl_106 {
() => {
// Module: crate::err
// Provides: {"impl_106"}
// Dependencies: {}
impl < T > TrySendError < T > { # [doc = " Unwraps the message."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_channel::bounded;"] # [doc = ""] # [doc = " let (s, r) = bounded(0);"] # [doc = ""] # [doc = " if let Err(err) = s.try_send(\"foo\") {"] # [doc = "     assert_eq!(err.into_inner(), \"foo\");"] # [doc = " }"] # [doc = " ```"] pub fn into_inner (self) -> T { match self { Self :: Full (v) => v , Self :: Disconnected (v) => v , } } # [doc = " Returns `true` if the send operation failed because the channel is full."] pub fn is_full (& self) -> bool { matches ! (self , Self :: Full (_)) } # [doc = " Returns `true` if the send operation failed because the channel is disconnected."] pub fn is_disconnected (& self) -> bool { matches ! (self , Self :: Disconnected (_)) } }
};
}
