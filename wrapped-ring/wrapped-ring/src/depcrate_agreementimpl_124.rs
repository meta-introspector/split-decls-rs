// Generated macro for impl_124 (impl)
macro_rules! Depcrate_agreementimpl_124 {
() => {
// Module: crate::agreement
// Provides: {"impl_124"}
// Dependencies: {}
impl EphemeralPrivateKey { # [doc = " Generate a new ephemeral private key for the given algorithm."] pub fn generate (alg : & 'static Algorithm , rng : & dyn rand :: SecureRandom ,) -> Result < Self , error :: Unspecified > { let cpu_features = cpu :: features () ; let private_key = ec :: Seed :: generate (alg . curve , rng , cpu_features) ? ; Ok (Self { private_key , algorithm : alg , }) } # [doc = " Computes the public key from the private key."] # [inline (always)] pub fn compute_public_key (& self) -> Result < PublicKey , error :: Unspecified > { self . private_key . compute_public_key (cpu :: features ()) . map (| public_key | PublicKey { algorithm : self . algorithm , bytes : public_key , }) } # [doc = " The algorithm for the private key."] # [inline] pub fn algorithm (& self) -> & 'static Algorithm { self . algorithm } # [doc = " Do not use."] # [deprecated] # [cfg (test)] pub fn bytes (& self) -> & [u8] { self . bytes_for_test () } # [cfg (test)] pub (super) fn bytes_for_test (& self) -> & [u8] { self . private_key . bytes_less_safe () } }
};
}
