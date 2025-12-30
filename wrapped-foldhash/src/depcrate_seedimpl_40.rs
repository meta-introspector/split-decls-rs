// Generated macro for impl_40 (impl)
macro_rules! Depcrate_seedimpl_40 {
() => {
// Module: crate::seed
// Provides: {"impl_40"}
// Dependencies: {}
impl SharedSeed { # [doc = " Returns the globally shared randomly initialized [`SharedSeed`] as used"] # [doc = " by [`RandomState`](crate::fast::RandomState)."] # [inline (always)] pub fn global_random () -> & 'static SharedSeed { global :: GlobalSeed :: new () . get () } # [doc = " Returns the globally shared fixed [`SharedSeed`] as used"] # [doc = " by [`FixedState`](crate::fast::FixedState)."] # [inline (always)] pub const fn global_fixed () -> & 'static SharedSeed { & FIXED_GLOBAL_SEED } # [doc = " Generates a new [`SharedSeed`] from a single 64-bit seed."] # [doc = ""] # [doc = " Note that this is somewhat expensive so it is suggested to re-use the"] # [doc = " [`SharedSeed`] as much as possible, using the per-hasher seed to"] # [doc = " differentiate between hash instances."] pub const fn from_u64 (seed : u64) -> Self { macro_rules ! mix { ($ x : expr) => { folded_multiply ($ x , ARBITRARY5) } ; } let seed_a = mix ! (mix ! (mix ! (seed))) ; let seed_b = mix ! (mix ! (mix ! (seed_a))) ; let seed_c = mix ! (mix ! (mix ! (seed_b))) ; let seed_d = mix ! (mix ! (mix ! (seed_c))) ; let seed_e = mix ! (mix ! (mix ! (seed_d))) ; let seed_f = mix ! (mix ! (mix ! (seed_e))) ; const FORCED_ONES : u64 = (1 << 63) | (1 << 31) | 1 ; Self { seeds : [seed_a | FORCED_ONES , seed_b | FORCED_ONES , seed_c | FORCED_ONES , seed_d | FORCED_ONES , seed_e | FORCED_ONES , seed_f | FORCED_ONES ,] , } } }
};
}
