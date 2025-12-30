// Generated macro for impl_413 (impl)
macro_rules! Depcrate_signatureimpl_413 {
() => {
// Module: crate::signature
// Provides: {"impl_413"}
// Dependencies: {}
impl AsDer < PublicKeyX509Der < 'static > > for ParsedPublicKey { fn as_der (& self) -> Result < PublicKeyX509Der < 'static > , Unspecified > { Ok (PublicKeyX509Der :: new (self . key . as_const () . marshal_rfc5280_public_key () ? ,)) } }
};
}
