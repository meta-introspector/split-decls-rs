// Generated macro for impl_675 (impl)
macro_rules! Depcrate_ed25519impl_675 {
() => {
// Module: crate::ed25519
// Provides: {"impl_675"}
// Dependencies: {}
impl AsDer < Pkcs8V2Der < 'static > > for Ed25519KeyPair { # [doc = " Serializes this `Ed25519KeyPair` into a PKCS#8 v1 document."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` on internal error."] fn as_der (& self) -> Result < Pkcs8V2Der < 'static > , crate :: error :: Unspecified > { Ok (Pkcs8V2Der :: new (self . evp_pkey . as_const () . marshal_rfc5208_private_key (Version :: V2) ? ,)) } }
};
}
