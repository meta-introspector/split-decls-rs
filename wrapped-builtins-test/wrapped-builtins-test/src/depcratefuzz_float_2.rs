// Generated macro for fuzz_float_2 (function)
macro_rules! Depcratefuzz_float_2 {
() => {
// Module: crate
// Provides: {"fuzz_float_2"}
// Dependencies: {}
pub fn fuzz_float_2 < F : Float , E : Fn (F , F) > (n : u32 , f : E) { float_edge_cases ! (F , case0 , { float_edge_cases ! (F , case1 , { f (case0 , case1) ; }) ; }) ; let mut rng = Xoshiro128StarStar :: seed_from_u64 (0) ; let mut x = F :: ZERO ; let mut y = F :: ZERO ; for _ in 0 .. n { fuzz_float_step (& mut rng , & mut x) ; fuzz_float_step (& mut rng , & mut y) ; f (x , y) } }
};
}
