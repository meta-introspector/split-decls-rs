// Generated macro for race_with_seed (function)
macro_rules! Depcrate_futurerace_with_seed {
() => {
// Module: crate::future
// Provides: {"race_with_seed"}
// Dependencies: {}
# [doc = " Race two futures but with a predefined random seed."] # [doc = ""] # [doc = " This function is identical to [`race`], but instead of using a random seed from a thread-local"] # [doc = " RNG, it allows the user to provide a seed. It is useful for when you already have a source of"] # [doc = " randomness available, or if you want to use a fixed seed."] # [doc = ""] # [doc = " See documentation of the [`race`] function for features and caveats."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::future::{self, pending, ready};"] # [doc = ""] # [doc = " // A fixed seed is used, so the result is deterministic."] # [doc = " const SEED: u64 = 0x42;"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " assert_eq!(future::race_with_seed(ready(1), pending(), SEED).await, 1);"] # [doc = " assert_eq!(future::race_with_seed(pending(), ready(2), SEED).await, 2);"] # [doc = ""] # [doc = " // One of the two futures is randomly chosen as the winner."] # [doc = " let res = future::race_with_seed(ready(1), ready(2), SEED).await;"] # [doc = " # })"] # [doc = " ```"] # [cfg (feature = "race")] pub fn race_with_seed < T , F1 , F2 > (future1 : F1 , future2 : F2 , seed : u64) -> Race < F1 , F2 > where F1 : Future < Output = T > , F2 : Future < Output = T > , { Race { future1 , future2 , rng : Rng :: with_seed (seed) , } }
};
}
