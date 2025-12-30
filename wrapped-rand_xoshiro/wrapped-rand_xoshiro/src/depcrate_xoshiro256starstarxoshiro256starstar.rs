// Generated macro for Xoshiro256StarStar (struct)
macro_rules! Depcrate_xoshiro256starstarXoshiro256StarStar {
() => {
// Module: crate::xoshiro256starstar
// Provides: {"Xoshiro256StarStar"}
// Dependencies: {}
# [doc = " A xoshiro256** random number generator."] # [doc = ""] # [doc = " The xoshiro256** algorithm is not suitable for cryptographic purposes, but"] # [doc = " is very fast and has excellent statistical properties."] # [doc = ""] # [doc = " The algorithm used here is translated from [the `xoshiro256starstar.c`"] # [doc = " reference source code](http://xoshiro.di.unimi.it/xoshiro256starstar.c) by"] # [doc = " David Blackman and Sebastiano Vigna."] # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Xoshiro256StarStar { s : [u64 ; 4] , }
};
}
