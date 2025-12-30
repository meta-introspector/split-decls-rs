// Generated macro for impl_429 (impl)
macro_rules! Depcrate_sync_mpscimpl_429 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_429"}
// Dependencies: {}
impl < T > Sender < T > { # [doc = " Attempts to send a value on this channel, returning it back if it could"] # [doc = " not be sent."] # [track_caller] pub fn send (& self , msg : T) -> Result < () , std :: sync :: mpsc :: SendError < T > > { self . object . send (location ! ()) ; self . sender . send (msg) } }
};
}
