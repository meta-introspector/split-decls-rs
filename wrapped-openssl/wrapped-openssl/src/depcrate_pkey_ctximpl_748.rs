// Generated macro for impl_748 (impl)
macro_rules! Depcrate_pkey_ctximpl_748 {
() => {
// Module: crate::pkey_ctx
// Provides: {"impl_748"}
// Dependencies: {}
# [cfg (ossl320)] impl NonceType { # [doc = " This is the default mode. It uses a random value for the nonce k as defined in FIPS 186-4 Section 6.3"] # [doc = " “Secret Number Generation”."] pub const RANDOM_K : Self = NonceType (0) ; # [doc = " Uses a deterministic value for the nonce k as defined in RFC #6979 (See Section 3.2 “Generation of k”)."] pub const DETERMINISTIC_K : Self = NonceType (1) ; }
};
}
