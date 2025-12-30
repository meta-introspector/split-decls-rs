// Generated macro for fuzz_shift (function)
macro_rules! Depcratefuzz_shift {
() => {
// Module: crate
// Provides: {"fuzz_shift"}
// Dependencies: {}
# [doc = " Tester for shift functions"] pub fn fuzz_shift < I : Int , F : Fn (I , u32) > (f : F) { let mut rng = Xoshiro128StarStar :: seed_from_u64 (0) ; let mut x : I = MinInt :: ZERO ; for i in 0 .. I :: FUZZ_NUM { fuzz_step (& mut rng , & mut x) ; f (x , MinInt :: ZERO) ; f (x , I :: FUZZ_LENGTHS [i] as u32) ; } }
};
}
