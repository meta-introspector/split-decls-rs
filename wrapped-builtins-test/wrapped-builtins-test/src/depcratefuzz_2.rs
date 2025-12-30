// Generated macro for fuzz_2 (function)
macro_rules! Depcratefuzz_2 {
() => {
// Module: crate
// Provides: {"fuzz_2"}
// Dependencies: {}
# [doc = " The same as `fuzz`, except `f` has two inputs."] pub fn fuzz_2 < I : Int , F : Fn (I , I) > (n : u32 , f : F) where < I as MinInt > :: Unsigned : Int , { edge_cases ! (I , case , { f (I :: ZERO , case) ; }) ; edge_cases ! (I , case , { f (case , I :: ZERO) ; }) ; edge_cases ! (I , case0 , { edge_cases ! (I , case1 , { f (case0 , case1) ; }) }) ; let mut rng = Xoshiro128StarStar :: seed_from_u64 (0) ; let mut x : I = I :: ZERO ; let mut y : I = I :: ZERO ; for _ in 0 .. n { fuzz_step (& mut rng , & mut x) ; fuzz_step (& mut rng , & mut y) ; f (x , y) } }
};
}
