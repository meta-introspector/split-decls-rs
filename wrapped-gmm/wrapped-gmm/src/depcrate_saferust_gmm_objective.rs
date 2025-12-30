// Generated macro for rust_gmm_objective (function)
macro_rules! Depcrate_saferust_gmm_objective {
() => {
// Module: crate::safe
// Provides: {"rust_gmm_objective"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn rust_gmm_objective (d : usize , k : usize , n : usize , alphas : * const f64 , means : * const f64 , icf : * const f64 , x : * const f64 , wishart : * const Wishart , err : * mut f64 ,) { let alphas = unsafe { std :: slice :: from_raw_parts (alphas , k) } ; let means = unsafe { std :: slice :: from_raw_parts (means , k * d) } ; let icf = unsafe { std :: slice :: from_raw_parts (icf , k * d * (d + 1) / 2) } ; let x = unsafe { std :: slice :: from_raw_parts (x , n * d) } ; let wishart : Wishart = unsafe { * wishart } ; let mut my_err = unsafe { * err } ; let (mut qdiags , mut sum_qs , mut xcentered , mut qxcentered , mut main_term) = get_workspace (d , k) ; gmm_objective (d , k , n , alphas , means , icf , x , wishart . gamma , wishart . m , & mut my_err , & mut qdiags , & mut sum_qs , & mut xcentered , & mut qxcentered , & mut main_term ,) ; unsafe { * err = my_err } ; }
};
}
