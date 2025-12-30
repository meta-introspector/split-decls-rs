// Generated macro for Xoroshiro128Plus (struct)
macro_rules! Depcrate_xoroshiro128plusXoroshiro128Plus {
() => {
// Module: crate::xoroshiro128plus
// Provides: {"Xoroshiro128Plus"}
// Dependencies: {}
# [doc = " A xoroshiro128+ random number generator."] # [doc = ""] # [doc = " The xoroshiro128+ algorithm is not suitable for cryptographic purposes, but"] # [doc = " is very fast and has good statistical properties, besides a low linear"] # [doc = " complexity in the lowest bits."] # [doc = ""] # [doc = " The algorithm used here is translated from [the `xoroshiro128plus.c`"] # [doc = " reference source code](http://xoshiro.di.unimi.it/xoroshiro128plus.c) by"] # [doc = " David Blackman and Sebastiano Vigna."] # [allow (missing_copy_implementations)] # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Xoroshiro128Plus { s0 : u64 , s1 : u64 , }
};
}
