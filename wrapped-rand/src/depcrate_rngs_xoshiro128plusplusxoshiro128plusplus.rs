// Generated macro for Xoshiro128PlusPlus (struct)
macro_rules! Depcrate_rngs_xoshiro128plusplusXoshiro128PlusPlus {
() => {
// Module: crate::rngs::xoshiro128plusplus
// Provides: {"Xoshiro128PlusPlus"}
// Dependencies: {}
# [doc = " A xoshiro128++ random number generator."] # [doc = ""] # [doc = " The xoshiro128++ algorithm is not suitable for cryptographic purposes, but"] # [doc = " is very fast and has excellent statistical properties."] # [doc = ""] # [doc = " The algorithm used here is translated from [the `xoshiro128plusplus.c`"] # [doc = " reference source code](http://xoshiro.di.unimi.it/xoshiro128plusplus.c) by"] # [doc = " David Blackman and Sebastiano Vigna."] # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Xoshiro128PlusPlus { s : [u32 ; 4] , }
};
}
