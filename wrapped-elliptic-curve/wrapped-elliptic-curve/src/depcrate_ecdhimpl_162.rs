// Generated macro for impl_162 (impl)
macro_rules! Depcrate_ecdhimpl_162 {
() => {
// Module: crate::ecdh
// Provides: {"impl_162"}
// Dependencies: {}
impl < C > EphemeralSecret < C > where C : CurveArithmetic , { # [doc = " Generate a cryptographically random [`EphemeralSecret`]."] # [cfg (feature = "getrandom")] pub fn generate () -> Self { Self { scalar : NonZeroScalar :: generate () , } } # [doc = " Generate a cryptographically random [`EphemeralSecret`]."] pub fn try_from_rng < R : TryCryptoRng + ? Sized > (rng : & mut R) -> Result < Self , R :: Error > { Ok (Self { scalar : NonZeroScalar :: try_from_rng (rng) ? , }) } # [doc = " Get the public key associated with this ephemeral secret."] # [doc = ""] # [doc = " The `compress` flag enables point compression."] pub fn public_key (& self) -> PublicKey < C > { PublicKey :: from_secret_scalar (& self . scalar) } # [doc = " Compute a Diffie-Hellman shared secret from an ephemeral secret and the"] # [doc = " public key of the other participant in the exchange."] pub fn diffie_hellman (& self , public_key : & PublicKey < C >) -> SharedSecret < C > { diffie_hellman (self . scalar , public_key . as_affine ()) } }
};
}
