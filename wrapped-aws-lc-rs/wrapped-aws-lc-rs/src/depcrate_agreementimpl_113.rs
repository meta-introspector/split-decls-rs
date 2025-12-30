// Generated macro for impl_113 (impl)
macro_rules! Depcrate_agreementimpl_113 {
() => {
// Module: crate::agreement
// Provides: {"impl_113"}
// Dependencies: {}
impl AsDer < PublicKeyX509Der < 'static > > for PublicKey { # [doc = " Provides the public key as a DER-encoded (X.509) `SubjectPublicKeyInfo` structure."] # [doc = " # Errors"] # [doc = " Returns an error if the public key fails to marshal to X.509."] fn as_der (& self) -> Result < PublicKeyX509Der < 'static > , crate :: error :: Unspecified > { match & self . inner_key { KeyInner :: ECDH_P256 (evp_pkey) | KeyInner :: ECDH_P384 (evp_pkey) | KeyInner :: ECDH_P521 (evp_pkey) | KeyInner :: X25519 (evp_pkey) => { let der = evp_pkey . as_const () . marshal_rfc5280_public_key () ? ; Ok (PublicKeyX509Der :: from (Buffer :: new (der))) } } } }
};
}
