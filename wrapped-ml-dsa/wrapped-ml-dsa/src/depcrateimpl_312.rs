// Generated macro for impl_312 (impl)
macro_rules! Depcrateimpl_312 {
() => {
// Module: crate
// Provides: {"impl_312"}
// Dependencies: {}
impl < P > KeyGen for P where P : MlDsaParams , { type KeyPair = KeyPair < P > ; # [doc = " Generate a signing key pair from the specified RNG"] # [cfg (feature = "rand_core")] fn key_gen < R : CryptoRng + ? Sized > (rng : & mut R) -> KeyPair < P > { let mut xi = B32 :: default () ; rng . fill_bytes (& mut xi) ; Self :: from_seed (& xi) } # [doc = " Deterministically generate a signing key pair from the specified seed"] # [doc = ""] # [doc = " This method reflects the ML-DSA.KeyGen_internal algorithm from FIPS 204."] fn from_seed (xi : & Seed) -> KeyPair < P > where P : MlDsaParams , { let mut h = H :: default () . absorb (xi) . absorb (& [P :: K :: U8]) . absorb (& [P :: L :: U8]) ; let rho : B32 = h . squeeze_new () ; let rhop : B64 = h . squeeze_new () ; let K : B32 = h . squeeze_new () ; let A_hat = expand_a :: < P :: K , P :: L > (& rho) ; let s1 = expand_s :: < P :: L > (& rhop , P :: Eta :: ETA , 0) ; let s2 = expand_s :: < P :: K > (& rhop , P :: Eta :: ETA , P :: L :: USIZE) ; let As1_hat = & A_hat * & s1 . ntt () ; let t = & As1_hat . ntt_inverse () + & s2 ; let (t1 , t0) = t . power2round () ; let verifying_key = VerifyingKey :: new (rho , t1 , Some (A_hat . clone ()) , None) ; let signing_key = SigningKey :: new (rho , K , verifying_key . tr . clone () , s1 , s2 , t0 , Some (A_hat)) ; KeyPair { signing_key , verifying_key , seed : xi . clone () , } } }
};
}
