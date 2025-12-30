// Generated macro for Sfc64 (struct)
macro_rules! Depcrate_sfc64Sfc64 {
() => {
// Module: crate::sfc64
// Provides: {"Sfc64"}
// Dependencies: {}
# [doc = " An sfc64 random number generator."] # [doc = ""] # [doc = " Good performance and statistical quality, but not cryptographically secure"] # [doc = " and has a large difference between its worst-case and maximum period."] # [doc = " sfc64 has a longer period than sfc32 and should perform as well or better"] # [doc = " on 64-bit processors, though Chris Doty-Humphrey believes its statistical"] # [doc = " properties are similar to sfc32."] # [doc = ""] # [doc = " This implementation is derived ultimately from"] # [doc = " [`the PractRand RNG test suite`](https://pracrand.sourceforge.net/) by"] # [doc = " Chris Doty-Humphrey."] # [allow (missing_copy_implementations)] # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Sfc64 { a : u64 , b : u64 , c : u64 , weyl : u64 , }
};
}
