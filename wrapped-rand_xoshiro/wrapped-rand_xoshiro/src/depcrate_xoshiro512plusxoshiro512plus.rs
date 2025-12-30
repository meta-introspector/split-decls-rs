// Generated macro for Xoshiro512Plus (struct)
macro_rules! Depcrate_xoshiro512plusXoshiro512Plus {
() => {
// Module: crate::xoshiro512plus
// Provides: {"Xoshiro512Plus"}
// Dependencies: {}
# [doc = " A xoshiro512+ random number generator."] # [doc = ""] # [doc = " The xoshiro512+ algorithm is not suitable for cryptographic purposes, but"] # [doc = " is very fast and has good statistical properties, besides a low linear"] # [doc = " complexity in the lowest bits."] # [doc = ""] # [doc = " The algorithm used here is translated from [the `xoshiro512plus.c`"] # [doc = " reference source code](http://xoshiro.di.unimi.it/xoshiro512plus.c) by"] # [doc = " David Blackman and Sebastiano Vigna."] # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Xoshiro512Plus { s : [u64 ; 8] , }
};
}
