// Generated macro for impl_239 (impl)
macro_rules! Depcrate_non_zeroimpl_239 {
() => {
// Module: crate::non_zero
// Provides: {"impl_239"}
// Dependencies: {}
# [cfg (feature = "rand_core")] impl < T > Random for NonZero < T > where T : Random + Zero , { # [doc = " This uses rejection sampling to avoid zero."] # [doc = ""] # [doc = " As a result, it runs in variable time. If the generator `rng` is"] # [doc = " cryptographically secure (for example, it implements `CryptoRng`),"] # [doc = " then this is guaranteed not to leak anything about the output value."] fn try_random < R : TryRngCore + ? Sized > (rng : & mut R) -> Result < Self , R :: Error > { loop { if let Some (result) = Self :: new (T :: try_random (rng) ?) . into () { break Ok (result) ; } } } }
};
}
