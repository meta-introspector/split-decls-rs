// Generated macro for Algorithm (struct)
macro_rules! Depcrate_agreementAlgorithm {
() => {
// Module: crate::agreement
// Provides: {"Algorithm"}
// Dependencies: {}
# [doc = " A key agreement algorithm."] pub struct Algorithm { pub (crate) curve : & 'static ec :: Curve , pub (crate) ecdh : fn (out : & mut [u8] , private_key : & ec :: Seed , peer_public_key : untrusted :: Input , cpu : cpu :: Features ,) -> Result < () , error :: Unspecified > , }
};
}
