// Generated macro for Xoroshiro64StarStar (struct)
macro_rules! Depcrate_xoroshiro64starstarXoroshiro64StarStar {
() => {
// Module: crate::xoroshiro64starstar
// Provides: {"Xoroshiro64StarStar"}
// Dependencies: {}
# [doc = " A xoroshiro64** random number generator."] # [doc = ""] # [doc = " The xoshiro64** algorithm is not suitable for cryptographic purposes, but"] # [doc = " is very fast and has excellent statistical properties."] # [doc = ""] # [doc = " The algorithm used here is translated from [the `xoroshiro64starstar.c`"] # [doc = " reference source code](http://xoshiro.di.unimi.it/xoroshiro64starstar.c) by"] # [doc = " David Blackman and Sebastiano Vigna."] # [allow (missing_copy_implementations)] # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Xoroshiro64StarStar { s0 : u32 , s1 : u32 , }
};
}
