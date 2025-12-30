// Generated macro for impl_84 (impl)
macro_rules! Depcrate_agreementimpl_84 {
() => {
// Module: crate::agreement
// Provides: {"impl_84"}
// Dependencies: {}
impl AlgorithmID { # [inline] const fn nid (& self) -> i32 { match self { AlgorithmID :: ECDH_P256 => NID_X9_62_prime256v1 , AlgorithmID :: ECDH_P384 => NID_secp384r1 , AlgorithmID :: ECDH_P521 => NID_secp521r1 , AlgorithmID :: X25519 => NID_X25519 , } } # [inline] const fn pub_key_len (& self) -> usize { match self { AlgorithmID :: ECDH_P256 => ec :: uncompressed_public_key_size_bytes (256) , AlgorithmID :: ECDH_P384 => ec :: uncompressed_public_key_size_bytes (384) , AlgorithmID :: ECDH_P521 => ec :: uncompressed_public_key_size_bytes (521) , AlgorithmID :: X25519 => 32 , } } # [inline] const fn private_key_len (& self) -> usize { match self { AlgorithmID :: ECDH_P256 | AlgorithmID :: X25519 => 32 , AlgorithmID :: ECDH_P384 => 48 , AlgorithmID :: ECDH_P521 => 66 , } } }
};
}
