// Generated macro for Xoshiro128StarStar (struct)
macro_rules! Depcrate_xoshiro128starstarXoshiro128StarStar {
() => {
// Module: crate::xoshiro128starstar
// Provides: {"Xoshiro128StarStar"}
// Dependencies: {}
# [doc = " A xoshiro128** random number generator."] # [doc = ""] # [doc = " The xoshiro128** algorithm is not suitable for cryptographic purposes, but"] # [doc = " is very fast and has excellent statistical properties."] # [doc = ""] # [doc = " The algorithm used here is translated from [the `xoshiro128starstar.c`"] # [doc = " reference source code](http://xoshiro.di.unimi.it/xoshiro128starstar.c) by"] # [doc = " David Blackman and Sebastiano Vigna."] # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Xoshiro128StarStar { s : [u32 ; 4] , }
};
}
