// Generated macro for KeyGen (trait)
macro_rules! DepcrateKeyGen {
() => {
// Module: crate
// Provides: {"KeyGen"}
// Dependencies: {}
# [doc = " A parameter set that knows how to generate key pairs"] pub trait KeyGen : MlDsaParams { # [doc = " The type that is returned by key generation"] type KeyPair : signature :: Keypair ; # [doc = " Generate a signing key pair from the specified RNG"] # [cfg (feature = "rand_core")] fn key_gen < R : CryptoRng + ? Sized > (rng : & mut R) -> Self :: KeyPair ; # [doc = " Deterministically generate a signing key pair from the specified seed"] # [doc = ""] # [doc = " This method reflects the ML-DSA.KeyGen_internal algorithm from FIPS 204."] fn from_seed (xi : & B32) -> Self :: KeyPair ; }
};
}
