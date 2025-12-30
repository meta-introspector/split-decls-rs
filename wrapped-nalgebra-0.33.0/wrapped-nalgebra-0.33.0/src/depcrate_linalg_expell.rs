// Generated macro for ell (function)
macro_rules! Depcrate_linalg_expell {
() => {
// Module: crate::linalg::exp
// Provides: {"ell"}
// Dependencies: {}
fn ell < T , D > (a : & OMatrix < T , D , D > , m : usize) -> u64 where T : ComplexField , D : Dim , DefaultAllocator : Allocator < D , D > + Allocator < D > + Allocator < D > + Allocator < D , D > , { let a_abs = a . map (| x | x . abs ()) ; let a_abs_onenorm = onenorm_matrix_power_nonm (& a_abs , 2 * m + 1) ; if a_abs_onenorm == < T as ComplexField > :: RealField :: zero () { return 0 ; } let m_factorial = factorial (m) ; let choose_2m_m = factorial (2 * m) / (m_factorial * m_factorial) ; let abs_c_recip = choose_2m_m * factorial (2 * m + 1) ; let alpha = a_abs_onenorm / one_norm (a) ; let alpha : f64 = try_convert :: < _ , f64 > (alpha) . unwrap () / abs_c_recip as f64 ; let u = 2_f64 . powf (- 53.0) ; let log2_alpha_div_u = (alpha / u) . log2 () ; let value = (log2_alpha_div_u / (2.0 * m as f64)) . ceil () ; if value > 0.0 { value as u64 } else { 0 } }
};
}
