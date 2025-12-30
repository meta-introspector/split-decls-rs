// Generated macro for Rand64 (struct)
macro_rules! DepcrateRand64 {
() => {
// Module: crate
// Provides: {"Rand64"}
// Dependencies: {}
# [doc = " A PRNG producing a 64-bit output."] # [doc = ""] # [doc = " The current implementation is `PCG-XSH-RR`."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct Rand64 { state : u128 , inc : u128 , }
};
}
