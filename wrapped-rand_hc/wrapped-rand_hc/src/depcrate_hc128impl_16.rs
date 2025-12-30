// Generated macro for impl_16 (impl)
macro_rules! Depcrate_hc128impl_16 {
() => {
// Module: crate::hc128
// Provides: {"impl_16"}
// Dependencies: {}
impl SeedableRng for Hc128Core { type Seed = [u8 ; SEED_WORDS * 4] ; # [doc = " Create an HC-128 random number generator with a seed. The seed has to be"] # [doc = " 256 bits in length, matching the 128 bit `key` followed by 128 bit `iv`"] # [doc = " when HC-128 where to be used as a stream cipher."] fn from_seed (seed : Self :: Seed) -> Self { let mut seed_u32 = [0u32 ; SEED_WORDS] ; le :: read_u32_into (& seed , & mut seed_u32) ; Self :: init (seed_u32) } }
};
}
