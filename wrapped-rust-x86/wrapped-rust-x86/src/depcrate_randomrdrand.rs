// Generated macro for RdRand (trait)
macro_rules! Depcrate_randomRdRand {
() => {
// Module: crate::random
// Provides: {"RdRand"}
// Dependencies: {}
# [doc = " RdRand trait to implement the generic rdrand_slice function."] pub trait RdRand { # [doc = " Fills `self` with random bits. Returns true on success or false otherwise"] # [doc = ""] # [doc = " # Safety"] # [doc = " RDRAND is not supported on all architctures, so using this may crash you."] unsafe fn fill_random (& mut self) -> bool ; }
};
}
