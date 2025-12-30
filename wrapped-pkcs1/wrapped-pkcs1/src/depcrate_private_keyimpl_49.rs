// Generated macro for impl_49 (impl)
macro_rules! Depcrate_private_keyimpl_49 {
() => {
// Module: crate::private_key
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a > RsaPrivateKey < 'a > { # [doc = " Get the public key that corresponds to this [`RsaPrivateKey`]."] pub fn public_key (& self) -> RsaPublicKey < 'a > { RsaPublicKey { modulus : self . modulus , public_exponent : self . public_exponent , } } # [doc = " Get the [`Version`] for this key."] # [doc = ""] # [doc = " Determined by the presence or absence of the"] # [doc = " [`RsaPrivateKey::other_prime_infos`] field."] pub fn version (& self) -> Version { if self . other_prime_infos . is_some () { Version :: Multi } else { Version :: TwoPrime } } }
};
}
