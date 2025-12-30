// Generated macro for impl_347 (impl)
macro_rules! Depcrate_hmacimpl_347 {
() => {
// Module: crate::hmac
// Provides: {"impl_347"}
// Dependencies: {}
impl From < hkdf :: Okm < '_ , Algorithm > > for Key { fn from (okm : hkdf :: Okm < Algorithm >) -> Self { Self :: construct (* okm . len () , | buf | okm . fill (buf) , cpu :: features ()) . unwrap () } }
};
}
