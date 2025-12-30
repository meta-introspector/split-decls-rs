// Generated macro for impl_258 (impl)
macro_rules! Depcrate_hkdfimpl_258 {
() => {
// Module: crate::hkdf
// Provides: {"impl_258"}
// Dependencies: {}
impl From < Okm < '_ , Algorithm > > for Prk { fn from (okm : Okm < Algorithm >) -> Self { let algorithm = okm . len ; let key_len = okm . len . len () ; let mut key_bytes = [0u8 ; MAX_HKDF_PRK_LEN] ; okm . fill (& mut key_bytes [0 .. key_len]) . unwrap () ; Self { algorithm , mode : PrkMode :: Expand { key_bytes , key_len } , } } }
};
}
