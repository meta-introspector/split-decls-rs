// Generated macro for Xoroshiro64Star (struct)
macro_rules! Depcrate_xoroshiro64starXoroshiro64Star {
() => {
// Module: crate::xoroshiro64star
// Provides: {"Xoroshiro64Star"}
// Dependencies: {}
# [doc = " A xoroshiro64* random number generator."] # [doc = ""] # [doc = " The xoroshiro64* algorithm is not suitable for cryptographic purposes, but"] # [doc = " is very fast and has good statistical properties, besides a low linear"] # [doc = " complexity in the lowest bits."] # [doc = ""] # [doc = " The algorithm used here is translated from [the `xoroshiro64star.c`"] # [doc = " reference source code](http://xoshiro.di.unimi.it/xoroshiro64star.c) by"] # [doc = " David Blackman and Sebastiano Vigna."] # [allow (missing_copy_implementations)] # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Xoroshiro64Star { s0 : u32 , s1 : u32 , }
};
}
