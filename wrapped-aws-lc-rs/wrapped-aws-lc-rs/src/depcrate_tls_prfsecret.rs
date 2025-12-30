// Generated macro for Secret (struct)
macro_rules! Depcrate_tls_prfSecret {
() => {
// Module: crate::tls_prf
// Provides: {"Secret"}
// Dependencies: {}
# [doc = " Encapsulates a PRF algorithm and secret bytes to be used to derive output."] pub struct Secret { algorithm : & 'static Algorithm , secret : Box < [u8] > , }
};
}
