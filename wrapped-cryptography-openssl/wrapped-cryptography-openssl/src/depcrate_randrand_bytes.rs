// Generated macro for rand_bytes (function)
macro_rules! Depcrate_randrand_bytes {
() => {
// Module: crate::rand
// Provides: {"rand_bytes"}
// Dependencies: {}
# [doc = " Fill a buffer with random bytes."] pub fn rand_bytes (buf : & mut [u8]) -> OpenSSLResult < () > { # [cfg (any (CRYPTOGRAPHY_IS_LIBRESSL , CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC))] openssl :: rand :: rand_bytes (buf) ? ; # [cfg (not (any (CRYPTOGRAPHY_IS_LIBRESSL , CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] openssl :: rand :: rand_priv_bytes (buf) ? ; Ok (()) }
};
}
