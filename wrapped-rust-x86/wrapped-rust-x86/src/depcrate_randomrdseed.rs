// Generated macro for RdSeed (trait)
macro_rules! Depcrate_randomRdSeed {
() => {
// Module: crate::random
// Provides: {"RdSeed"}
// Dependencies: {}
# [doc = " RdSeed trait to implement the generic rdseed_slice function."] pub trait RdSeed { # [doc = " Fills `self` with random bits. Returns true on success or false otherwise"] # [doc = ""] # [doc = " # Safety"] # [doc = " RDSEED is not supported on all architctures, so using this may crash you."] unsafe fn fill_random (& mut self) -> bool ; }
};
}
