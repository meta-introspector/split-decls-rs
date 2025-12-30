// Generated macro for Seed512 (struct)
macro_rules! Depcrate_commonSeed512 {
() => {
// Module: crate::common
// Provides: {"Seed512"}
// Dependencies: {}
# [doc = " 512-bit seed for a generator."] # [doc = ""] # [doc = " This wrapper is necessary, because some traits required for a seed are not"] # [doc = " implemented on large arrays."] # [derive (Clone)] pub struct Seed512 (pub [u8 ; 64]) ;
};
}
