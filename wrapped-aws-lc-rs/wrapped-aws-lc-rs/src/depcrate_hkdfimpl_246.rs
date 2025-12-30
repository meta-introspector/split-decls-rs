// Generated macro for impl_246 (impl)
macro_rules! Depcrate_hkdfimpl_246 {
() => {
// Module: crate::hkdf
// Provides: {"impl_246"}
// Dependencies: {}
impl From < Okm < '_ , Algorithm > > for Salt { fn from (okm : Okm < '_ , Algorithm >) -> Self { let algorithm = okm . prk . algorithm ; let mut salt_bytes = [0u8 ; MAX_HKDF_SALT_LEN] ; let salt_len = okm . len () . len () ; okm . fill (& mut salt_bytes [.. salt_len]) . unwrap () ; Self { algorithm , bytes : salt_bytes , len : salt_len , } } }
};
}
