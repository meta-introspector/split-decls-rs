// Generated macro for impl_324 (impl)
macro_rules! Depcrate_hkdfimpl_324 {
() => {
// Module: crate::hkdf
// Provides: {"impl_324"}
// Dependencies: {}
impl From < Okm < '_ , Algorithm > > for Salt { fn from (okm : Okm < '_ , Algorithm >) -> Self { Self (hmac :: Key :: from (Okm { prk : okm . prk , info : okm . info , len : okm . len () . 0 , len_cached : okm . len_cached , })) } }
};
}
