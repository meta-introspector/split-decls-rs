// Generated macro for impl_61 (impl)
macro_rules! Depcrate_hmacimpl_61 {
() => {
// Module: crate::hmac
// Provides: {"impl_61"}
// Dependencies: {}
impl crypto :: hmac :: Key for Sha256HmacKey { fn sign_concat (& self , first : & [u8] , middle : & [& [u8]] , last : & [u8]) -> crypto :: hmac :: Tag { let mut ctx = self . 0 . clone () ; ctx . update (first) ; for m in middle { ctx . update (m) ; } ctx . update (last) ; crypto :: hmac :: Tag :: new (& ctx . finalize () . into_bytes () [..]) } fn tag_len (& self) -> usize { Sha256 :: output_size () } }
};
}
