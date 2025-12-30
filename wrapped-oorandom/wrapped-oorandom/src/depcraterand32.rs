// Generated macro for Rand32 (struct)
macro_rules! DepcrateRand32 {
() => {
// Module: crate
// Provides: {"Rand32"}
// Dependencies: {}
# [doc = " A PRNG producing a 32-bit output."] # [doc = ""] # [doc = " The current implementation is `PCG-XSH-RR`."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct Rand32 { state : u64 , inc : u64 , }
};
}
