// Generated macro for impl_674 (impl)
macro_rules! Depcrate_ed25519impl_674 {
() => {
// Module: crate::ed25519
// Provides: {"impl_674"}
// Dependencies: {}
impl AsDer < Pkcs8V1Der < 'static > > for Ed25519KeyPair { # [doc = " Serializes this `Ed25519KeyPair` into a PKCS#8 v1 document."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` on internal error."] fn as_der (& self) -> Result < Pkcs8V1Der < 'static > , crate :: error :: Unspecified > { Ok (Pkcs8V1Der :: new (self . evp_pkey . as_const () . marshal_rfc5208_private_key (Version :: V1) ? ,)) } }
};
}
