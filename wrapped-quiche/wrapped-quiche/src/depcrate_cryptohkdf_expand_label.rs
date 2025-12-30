// Generated macro for hkdf_expand_label (function)
macro_rules! Depcrate_cryptohkdf_expand_label {
() => {
// Module: crate::crypto
// Provides: {"hkdf_expand_label"}
// Dependencies: {}
fn hkdf_expand_label (alg : Algorithm , prk : & [u8] , label : & [u8] , out : & mut [u8] ,) -> Result < () > { const LABEL_PREFIX : & [u8] = b"tls13 " ; let out_len = (out . len () as u16) . to_be_bytes () ; let label_len = (LABEL_PREFIX . len () + label . len ()) as u8 ; let info = [& out_len , & [label_len] [..] , LABEL_PREFIX , label , & [0] [..]] ; let info = info . concat () ; hkdf_expand (alg , out , prk , & info) ? ; Ok (()) }
};
}
