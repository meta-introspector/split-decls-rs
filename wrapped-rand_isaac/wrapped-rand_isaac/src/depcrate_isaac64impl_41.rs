// Generated macro for impl_41 (impl)
macro_rules! Depcrate_isaac64impl_41 {
() => {
// Module: crate::isaac64
// Provides: {"impl_41"}
// Dependencies: {}
impl SeedableRng for Isaac64Core { type Seed = [u8 ; 32] ; fn from_seed (seed : Self :: Seed) -> Self { let mut seed_u64 = [0u64 ; 4] ; le :: read_u64_into (& seed , & mut seed_u64) ; let mut seed_extended = [w (0) ; RAND_SIZE] ; for (x , y) in seed_extended . iter_mut () . zip (seed_u64 . iter ()) { * x = w (* y) ; } Self :: init (seed_extended , 2) } fn seed_from_u64 (seed : u64) -> Self { let mut key = [w (0) ; RAND_SIZE] ; key [0] = w (seed) ; Self :: init (key , 1) } fn from_rng < R > (rng : & mut R) -> Self where R : RngCore + ? Sized , { let mut seed = [w (0u64) ; RAND_SIZE] ; unsafe { let ptr = seed . as_mut_ptr () as * mut u8 ; let slice = slice :: from_raw_parts_mut (ptr , RAND_SIZE * 8) ; rng . fill_bytes (slice) ; } for i in seed . iter_mut () { * i = w (i . 0 . to_le ()) ; } Self :: init (seed , 2) } fn try_from_rng < R > (rng : & mut R) -> Result < Self , R :: Error > where R : TryRngCore + ? Sized , { let mut seed = [w (0u64) ; RAND_SIZE] ; unsafe { let ptr = seed . as_mut_ptr () as * mut u8 ; let slice = slice :: from_raw_parts_mut (ptr , RAND_SIZE * 8) ; rng . try_fill_bytes (slice) ? ; } for i in seed . iter_mut () { * i = w (i . 0 . to_le ()) ; } Ok (Self :: init (seed , 2)) } }
};
}
