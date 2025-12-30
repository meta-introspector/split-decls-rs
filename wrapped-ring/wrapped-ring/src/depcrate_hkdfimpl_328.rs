// Generated macro for impl_328 (impl)
macro_rules! Depcrate_hkdfimpl_328 {
() => {
// Module: crate::hkdf
// Provides: {"impl_328"}
// Dependencies: {}
impl From < Okm < '_ , Algorithm > > for Prk { fn from (okm : Okm < Algorithm >) -> Self { Self (hmac :: Key :: from (Okm { prk : okm . prk , info : okm . info , len : okm . len () . 0 , len_cached : okm . len_cached , })) } }
};
}
