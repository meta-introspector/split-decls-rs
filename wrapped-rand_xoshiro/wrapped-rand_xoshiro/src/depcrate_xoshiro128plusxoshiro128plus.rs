// Generated macro for Xoshiro128Plus (struct)
macro_rules! Depcrate_xoshiro128plusXoshiro128Plus {
() => {
// Module: crate::xoshiro128plus
// Provides: {"Xoshiro128Plus"}
// Dependencies: {}
# [doc = " A xoshiro128+ random number generator."] # [doc = ""] # [doc = " The xoshiro128+ algorithm is not suitable for cryptographic purposes, but"] # [doc = " is very fast and has good statistical properties, besides a low linear"] # [doc = " complexity in the lowest bits."] # [doc = ""] # [doc = " The algorithm used here is translated from [the `xoshiro128starstar.c`"] # [doc = " reference source code](http://xoshiro.di.unimi.it/xoshiro128starstar.c) by"] # [doc = " David Blackman and Sebastiano Vigna."] # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Xoshiro128Plus { s : [u32 ; 4] , }
};
}
