// Generated macro for bn_to_big_endian_bytes (function)
macro_rules! Depcrate_utilsbn_to_big_endian_bytes {
() => {
// Module: crate::utils
// Provides: {"bn_to_big_endian_bytes"}
// Dependencies: {}
pub fn bn_to_big_endian_bytes (b : & openssl :: bn :: BigNumRef) -> OpenSSLResult < Vec < u8 > > { b . to_vec_padded (b . num_bits () / 8 + 1) }
};
}
