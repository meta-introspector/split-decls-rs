// Generated macro for Xoshiro512PlusPlus (struct)
macro_rules! Depcrate_xoshiro512plusplusXoshiro512PlusPlus {
() => {
// Module: crate::xoshiro512plusplus
// Provides: {"Xoshiro512PlusPlus"}
// Dependencies: {}
# [doc = " A xoshiro512++ random number generator."] # [doc = ""] # [doc = " The xoshiro512++ algorithm is not suitable for cryptographic purposes, but"] # [doc = " is very fast and has excellent statistical properties."] # [doc = ""] # [doc = " The algorithm used here is translated from [the `xoshiro512plusplus.c`"] # [doc = " reference source code](http://xoshiro.di.unimi.it/xoshiro512plusplus.c) by"] # [doc = " David Blackman and Sebastiano Vigna."] # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Xoshiro512PlusPlus { s : [u64 ; 8] , }
};
}
