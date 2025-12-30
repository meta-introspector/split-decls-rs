// Generated macro for fuzz (function)
macro_rules! Depcratefuzz {
() => {
// Module: crate
// Provides: {"fuzz"}
// Dependencies: {}
# [doc = " Feeds a series of fuzzing inputs to `f`. The fuzzer first uses an algorithm designed to find"] # [doc = " edge cases, followed by a more random fuzzer that runs `n` times."] pub fn fuzz < I : Int , F : FnMut (I) > (n : u32 , mut f : F) where < I as MinInt > :: Unsigned : Int , { f (I :: ZERO) ; edge_cases ! (I , case , { f (case) ; }) ; let mut rng = Xoshiro128StarStar :: seed_from_u64 (0) ; let mut x : I = MinInt :: ZERO ; for _ in 0 .. n { fuzz_step (& mut rng , & mut x) ; f (x) } }
};
}
