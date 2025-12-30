// Generated macro for create_initial_generation (function)
macro_rules! Depcratecreate_initial_generation {
() => {
// Module: crate
// Provides: {"create_initial_generation"}
// Dependencies: {}
# [doc = " Creates the initial generation seeded by the current time."] # [cfg_attr (mutants , mutants :: skip)] # [must_use] fn create_initial_generation () -> u64 { # [cfg (feature = "std")] { use std :: { collections :: hash_map :: RandomState , hash :: BuildHasher } ; let mut hasher = RandomState :: new () . build_hasher () ; hasher . write_u32 (0) ; hasher . finish () } # [cfg (not (feature = "std"))] { use core :: sync :: atomic :: { AtomicU32 , Ordering } ; fn gen_u32 () -> u32 { static SEED : AtomicU32 = AtomicU32 :: new ({ const_random :: const_random ! (u32) }) ; let mut x = SEED . load (Ordering :: Relaxed) ; loop { let mut random = x ; random ^= random << 13 ; random ^= random >> 17 ; random ^= random << 5 ; if let Err (actual) = SEED . compare_exchange (x , random , Ordering :: SeqCst , Ordering :: SeqCst) { x = actual ; } else { return random ; } } } gen_u32 () as u64 | ((gen_u32 () as u64) << 32) } }
};
}
