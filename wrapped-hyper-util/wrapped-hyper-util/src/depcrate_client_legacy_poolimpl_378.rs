// Generated macro for impl_378 (impl)
macro_rules! Depcrate_client_legacy_poolimpl_378 {
() => {
// Module: crate::client::legacy::pool
// Provides: {"impl_378"}
// Dependencies: {}
impl Expiration { fn new (dur : Option < Duration >) -> Expiration { Expiration (dur) } fn expires (& self , instant : Instant , now : Instant) -> bool { match self . 0 { Some (timeout) => now . saturating_duration_since (instant) > timeout , None => false , } } }
};
}
