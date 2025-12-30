// Generated macro for Xoshiro256Plus (struct)
macro_rules! Depcrate_xoshiro256plusXoshiro256Plus {
() => {
// Module: crate::xoshiro256plus
// Provides: {"Xoshiro256Plus"}
// Dependencies: {}
# [doc = " A xoshiro256+ random number generator."] # [doc = ""] # [doc = " The xoshiro256+ algorithm is not suitable for cryptographic purposes, but"] # [doc = " is very fast and has good statistical properties, besides a low linear"] # [doc = " complexity in the lowest bits."] # [doc = ""] # [doc = " The algorithm used here is translated from [the `xoshiro256plus.c`"] # [doc = " reference source code](http://xoshiro.di.unimi.it/xoshiro256plus.c) by"] # [doc = " David Blackman and Sebastiano Vigna."] # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Xoshiro256Plus { s : [u64 ; 4] , }
};
}
