// Generated macro for impl_143 (impl)
macro_rules! Depcrate_binomialimpl_143 {
() => {
// Module: crate::binomial
// Provides: {"impl_143"}
// Dependencies: {}
impl Binomial { # [doc = " Construct a new `Binomial` with the given shape parameters `n` (number"] # [doc = " of trials) and `p` (probability of success)."] pub fn new (n : u64 , p : f64) -> Result < Binomial , Error > { if ! (p >= 0.0) { return Err (Error :: ProbabilityTooSmall) ; } if ! (p <= 1.0) { return Err (Error :: ProbabilityTooLarge) ; } if p == 0.0 { return Ok (Binomial { method : Method :: Constant (0) , }) ; } if p == 1.0 { return Ok (Binomial { method : Method :: Constant (n) , }) ; } let flipped = p > 0.5 ; let p = if flipped { 1.0 - p } else { p } ; const BINV_THRESHOLD : f64 = 10. ; let np = n as f64 * p ; let method = if np < BINV_THRESHOLD { let q = 1.0 - p ; if q == 1.0 { Method :: Poisson (crate :: poisson :: KnuthMethod :: new (np)) } else { let s = p / q ; Method :: Binv (Binv { r : q . powf (n as f64) , s , a : (n as f64 + 1.0) * s , n , } , flipped ,) } } else { let q = 1.0 - p ; let npq = np * q ; let p1 = (2.195 * npq . sqrt () - 4.6 * q) . floor () + 0.5 ; let f_m = np + p ; let m = f64_to_i64 (f_m) ; Method :: Btpe (Btpe { n , p , m , p1 } , flipped) } ; Ok (Binomial { method }) } }
};
}
