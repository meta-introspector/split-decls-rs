// Generated macro for impl_59 (impl)
macro_rules! Depcrate_hmacimpl_59 {
() => {
// Module: crate::hmac
// Provides: {"impl_59"}
// Dependencies: {}
impl crypto :: hmac :: Hmac for Sha256Hmac { fn with_key (& self , key : & [u8]) -> Box < dyn crypto :: hmac :: Key > { Box :: new (Sha256HmacKey (Hmac :: < Sha256 > :: new_from_slice (key) . unwrap ())) } fn hash_output_len (& self) -> usize { Sha256 :: output_size () } }
};
}
