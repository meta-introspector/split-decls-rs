// Generated macro for rej_ntt_poly (function)
macro_rules! Depcrate_samplingrej_ntt_poly {
() => {
// Module: crate::sampling
// Provides: {"rej_ntt_poly"}
// Dependencies: {}
fn rej_ntt_poly (rho : & [u8] , r : u8 , s : u8) -> NttPolynomial { let mut j = 0 ; let mut ctx = G :: default () . absorb (rho) . absorb (& [s]) . absorb (& [r]) ; let mut a = NttPolynomial :: default () ; let mut s = [0u8 ; 3] ; while j < 256 { ctx . squeeze (& mut s) ; if let Some (x) = coeff_from_three_bytes (s) { a . 0 [j] = x ; j += 1 ; } } a }
};
}
