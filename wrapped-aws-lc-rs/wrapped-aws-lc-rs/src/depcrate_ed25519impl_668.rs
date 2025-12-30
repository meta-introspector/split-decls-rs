// Generated macro for impl_668 (impl)
macro_rules! Depcrate_ed25519impl_668 {
() => {
// Module: crate::ed25519
// Provides: {"impl_668"}
// Dependencies: {}
impl AsDer < PublicKeyX509Der < 'static > > for PublicKey { # [doc = " Provides the public key as a DER-encoded (X.509) `SubjectPublicKeyInfo` structure."] # [doc = " # Errors"] # [doc = " Returns an error if the public key fails to marshal to X.509."] fn as_der (& self) -> Result < PublicKeyX509Der < 'static > , crate :: error :: Unspecified > { let der = self . evp_pkey . as_const () . marshal_rfc5280_public_key () ? ; Ok (PublicKeyX509Der :: from (Buffer :: new (der))) } }
};
}
