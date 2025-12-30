// Generated macro for Xoroshiro128PlusPlus (struct)
macro_rules! Depcrate_xoroshiro128plusplusXoroshiro128PlusPlus {
() => {
// Module: crate::xoroshiro128plusplus
// Provides: {"Xoroshiro128PlusPlus"}
// Dependencies: {}
# [doc = " A xoroshiro128++ random number generator."] # [doc = ""] # [doc = " The xoroshiro128++ algorithm is not suitable for cryptographic purposes, but"] # [doc = " is very fast and has excellent statistical properties."] # [doc = ""] # [doc = " The algorithm used here is translated from [the `xoroshiro128plusplus.c`"] # [doc = " reference source code](http://xoshiro.di.unimi.it/xoroshiro128plusplus.c) by"] # [doc = " David Blackman and Sebastiano Vigna."] # [allow (missing_copy_implementations)] # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Xoroshiro128PlusPlus { s0 : u64 , s1 : u64 , }
};
}
