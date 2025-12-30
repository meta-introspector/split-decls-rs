// Generated macro for impl_296 (impl)
macro_rules! Depcrate_hmacimpl_296 {
() => {
// Module: crate::hmac
// Provides: {"impl_296"}
// Dependencies: {}
impl From < hkdf :: Okm < '_ , Algorithm > > for Key { fn from (okm : hkdf :: Okm < Algorithm >) -> Self { Self :: construct (* okm . len () , | buf | okm . fill (buf)) . unwrap () } }
};
}
