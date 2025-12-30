// Generated macro for impl_508 (impl)
macro_rules! Depcrate_wrappingimpl_508 {
() => {
// Module: crate::wrapping
// Provides: {"impl_508"}
// Dependencies: {}
# [cfg (feature = "rand_core")] impl < T : Random > Random for Wrapping < T > { fn try_random < R : TryRngCore + ? Sized > (rng : & mut R) -> Result < Self , R :: Error > { Ok (Wrapping (Random :: try_random (rng) ?)) } }
};
}
