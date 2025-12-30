// Generated macro for impl_298 (impl)
macro_rules! Depcrate_oddimpl_298 {
() => {
// Module: crate::odd
// Provides: {"impl_298"}
// Dependencies: {}
# [cfg (all (feature = "alloc" , feature = "rand_core"))] impl Odd < BoxedUint > { # [doc = " Generate a random `Odd<Uint<T>>`."] pub fn random < R : TryRngCore + ? Sized > (rng : & mut R , bit_length : u32) -> Self { let mut ret = BoxedUint :: random_bits (rng , bit_length) ; ret . limbs [0] |= Limb :: ONE ; Odd (ret) } }
};
}
