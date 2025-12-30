// Generated macro for SplitMix64 (struct)
macro_rules! Depcrate_splitmix64SplitMix64 {
() => {
// Module: crate::splitmix64
// Provides: {"SplitMix64"}
// Dependencies: {}
# [doc = " A splitmix64 random number generator."] # [doc = ""] # [doc = " The splitmix algorithm is not suitable for cryptographic purposes, but is"] # [doc = " very fast and has a 64 bit state."] # [doc = ""] # [doc = " The algorithm used here is translated from [the `splitmix64.c`"] # [doc = " reference source code](http://xoshiro.di.unimi.it/splitmix64.c) by"] # [doc = " Sebastiano Vigna. For `next_u32`, a more efficient mixing function taken"] # [doc = " from [`dsiutils`](http://dsiutils.di.unimi.it/) is used."] # [allow (missing_copy_implementations)] # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct SplitMix64 { x : u64 , }
};
}
