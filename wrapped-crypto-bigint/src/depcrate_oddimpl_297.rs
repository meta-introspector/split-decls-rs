// Generated macro for impl_297 (impl)
macro_rules! Depcrate_oddimpl_297 {
() => {
// Module: crate::odd
// Provides: {"impl_297"}
// Dependencies: {}
# [cfg (feature = "rand_core")] impl < const LIMBS : usize > Random for Odd < Uint < LIMBS > > { # [doc = " Generate a random `Odd<Uint<T>>`."] fn try_random < R : TryRngCore + ? Sized > (rng : & mut R) -> Result < Self , R :: Error > { let mut ret = Uint :: try_random (rng) ? ; ret . limbs [0] |= Limb :: ONE ; Ok (Odd (ret)) } }
};
}
