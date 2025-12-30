// Generated macro for Xoshiro512StarStar (struct)
macro_rules! Depcrate_xoshiro512starstarXoshiro512StarStar {
() => {
// Module: crate::xoshiro512starstar
// Provides: {"Xoshiro512StarStar"}
// Dependencies: {}
# [doc = " A xoshiro512** random number generator."] # [doc = ""] # [doc = " The xoshiro512** algorithm is not suitable for cryptographic purposes, but"] # [doc = " is very fast and has excellent statistical properties."] # [doc = ""] # [doc = " The algorithm used here is translated from [the `xoshiro512starstar.c`"] # [doc = " reference source code](http://xoshiro.di.unimi.it/xoshiro512starstar.c) by"] # [doc = " David Blackman and Sebastiano Vigna."] # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Xoshiro512StarStar { s : [u64 ; 8] , }
};
}
