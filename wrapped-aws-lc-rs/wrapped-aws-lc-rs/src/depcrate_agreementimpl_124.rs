// Generated macro for impl_124 (impl)
macro_rules! Depcrate_agreementimpl_124 {
() => {
// Module: crate::agreement
// Provides: {"impl_124"}
// Dependencies: {}
# [doc = " A parsed public key for key agreement."] impl ParsedPublicKey { fn nid (& self) -> i32 { self . nid } # [doc = " The format of the data the public key was parsed from."] # [must_use] pub fn format (& self) -> ParsedPublicKeyFormat { self . format } pub (crate) fn key (& self) -> & LcPtr < EVP_PKEY > { & self . key } # [doc = " The algorithm of the public key."] # [must_use] # [allow (non_upper_case_globals)] pub fn alg (& self) -> & 'static Algorithm { match self . nid () { NID_X25519 => & X25519 , NID_X9_62_prime256v1 => & ECDH_P256 , NID_secp384r1 => & ECDH_P384 , NID_secp521r1 => & ECDH_P521 , _ => unreachable ! ("Unreachable agreement algorithm nid: {}" , self . nid ()) , } } }
};
}
