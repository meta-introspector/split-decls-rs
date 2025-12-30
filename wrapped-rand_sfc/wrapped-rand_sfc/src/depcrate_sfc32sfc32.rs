// Generated macro for Sfc32 (struct)
macro_rules! Depcrate_sfc32Sfc32 {
() => {
// Module: crate::sfc32
// Provides: {"Sfc32"}
// Dependencies: {}
# [doc = " An sfc32 random number generator."] # [doc = ""] # [doc = " Good performance and statistical quality, but not cryptographically secure"] # [doc = " and has a large difference between its worst-case and maximum period."] # [doc = ""] # [doc = " This implementation is derived ultimately from"] # [doc = " [`the PractRand RNG test suite`](https://pracrand.sourceforge.net/) by"] # [doc = " Chris Doty-Humphrey."] # [allow (missing_copy_implementations)] # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Sfc32 { a : u32 , b : u32 , c : u32 , weyl : u32 , }
};
}
