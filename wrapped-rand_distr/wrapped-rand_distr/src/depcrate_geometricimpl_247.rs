// Generated macro for impl_247 (impl)
macro_rules! Depcrate_geometricimpl_247 {
() => {
// Module: crate::geometric
// Provides: {"impl_247"}
// Dependencies: {}
impl Geometric { # [doc = " Construct a new `Geometric` with the given shape parameter `p`"] # [doc = " (probability of success on each trial)."] pub fn new (p : f64) -> Result < Self , Error > { if ! p . is_finite () || ! (0.0 ..= 1.0) . contains (& p) { Err (Error :: InvalidProbability) } else if p == 0.0 || p >= 2.0 / 3.0 { Ok (Geometric { p , pi : p , k : 0 }) } else { let (pi , k) = { let mut k = 1 ; let mut pi = (1.0 - p) . powi (2) ; while pi > 0.5 { k += 1 ; pi = pi * pi ; } (pi , k) } ; Ok (Geometric { p , pi , k }) } } }
};
}
