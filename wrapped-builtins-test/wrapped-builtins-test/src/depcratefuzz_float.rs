// Generated macro for fuzz_float (function)
macro_rules! Depcratefuzz_float {
() => {
// Module: crate
// Provides: {"fuzz_float"}
// Dependencies: {}
pub fn fuzz_float < F : Float , E : Fn (F) > (n : u32 , f : E) { float_edge_cases ! (F , case , { f (case) ; }) ; let mut rng = Xoshiro128StarStar :: seed_from_u64 (0) ; let mut x = F :: ZERO ; for _ in 0 .. n { fuzz_float_step (& mut rng , & mut x) ; f (x) ; } }
};
}
