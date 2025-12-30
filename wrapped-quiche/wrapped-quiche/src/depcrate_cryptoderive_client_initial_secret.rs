// Generated macro for derive_client_initial_secret (function)
macro_rules! Depcrate_cryptoderive_client_initial_secret {
() => {
// Module: crate::crypto
// Provides: {"derive_client_initial_secret"}
// Dependencies: {}
fn derive_client_initial_secret (aead : Algorithm , prk : & [u8] , out : & mut [u8] ,) -> Result < () > { const LABEL : & [u8] = b"client in" ; hkdf_expand_label (aead , prk , LABEL , out) }
};
}
