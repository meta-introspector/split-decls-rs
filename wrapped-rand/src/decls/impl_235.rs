macro_rules! deps {
    () => {
        Xoshiro128PlusPlus!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl SeedableRng for Xoshiro128PlusPlus { type Seed = [u8 ; 16] ; # [doc = " Create a new `Xoshiro128PlusPlus`.  If `seed` is entirely 0, it will be"] # [doc = " mapped to a different seed."] # [inline] fn from_seed (seed : [u8 ; 16]) -> Xoshiro128PlusPlus { let mut state = [0 ; 4] ; le :: read_u32_into (& seed , & mut state) ; if state . iter () . all (| & x | x == 0) { return Self :: seed_from_u64 (0) ; } Xoshiro128PlusPlus { s : state } } # [doc = " Create a new `Xoshiro128PlusPlus` from a `u64` seed."] # [doc = ""] # [doc = " This uses the SplitMix64 generator internally."] # [inline] fn seed_from_u64 (mut state : u64) -> Self { const PHI : u64 = 0x9e3779b97f4a7c15 ; let mut s = [0 ; 4] ; for i in s . chunks_exact_mut (2) { state = state . wrapping_add (PHI) ; let mut z = state ; z = (z ^ (z >> 30)) . wrapping_mul (0xbf58476d1ce4e5b9) ; z = (z ^ (z >> 27)) . wrapping_mul (0x94d049bb133111eb) ; z = z ^ (z >> 31) ; i [0] = z as u32 ; i [1] = (z >> 32) as u32 ; } debug_assert_ne ! (s , [0 ; 4]) ; Xoshiro128PlusPlus { s } } }
    };
}

impl_235!()