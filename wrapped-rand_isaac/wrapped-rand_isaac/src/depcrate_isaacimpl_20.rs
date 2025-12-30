// Generated macro for impl_20 (impl)
macro_rules! Depcrate_isaacimpl_20 {
() => {
// Module: crate::isaac
// Provides: {"impl_20"}
// Dependencies: {}
impl SeedableRng for IsaacCore { type Seed = [u8 ; 32] ; fn from_seed (seed : Self :: Seed) -> Self { let mut seed_u32 = [0u32 ; 8] ; le :: read_u32_into (& seed , & mut seed_u32) ; let mut seed_extended = [w (0) ; RAND_SIZE] ; for (x , y) in seed_extended . iter_mut () . zip (seed_u32 . iter ()) { * x = w (* y) ; } Self :: init (seed_extended , 2) } # [doc = " Create an ISAAC random number generator using an `u64` as seed."] # [doc = " If `seed == 0` this will produce the same stream of random numbers as"] # [doc = " the reference implementation when used unseeded."] fn seed_from_u64 (seed : u64) -> Self { let mut key = [w (0) ; RAND_SIZE] ; key [0] = w (seed as u32) ; key [1] = w ((seed >> 32) as u32) ; Self :: init (key , 1) } fn from_rng < R > (rng : & mut R) -> Self where R : RngCore + ? Sized , { let mut seed = [w (0u32) ; RAND_SIZE] ; unsafe { let ptr = seed . as_mut_ptr () as * mut u8 ; let slice = slice :: from_raw_parts_mut (ptr , RAND_SIZE * 4) ; rng . fill_bytes (slice) ; } for i in seed . iter_mut () { * i = w (i . 0 . to_le ()) ; } Self :: init (seed , 2) } fn try_from_rng < R > (rng : & mut R) -> Result < Self , R :: Error > where R : TryRngCore + ? Sized , { let mut seed = [w (0u32) ; RAND_SIZE] ; unsafe { let ptr = seed . as_mut_ptr () as * mut u8 ; let slice = slice :: from_raw_parts_mut (ptr , RAND_SIZE * 4) ; rng . try_fill_bytes (slice) ? ; } for i in seed . iter_mut () { * i = w (i . 0 . to_le ()) ; } Ok (Self :: init (seed , 2)) } }
};
}
