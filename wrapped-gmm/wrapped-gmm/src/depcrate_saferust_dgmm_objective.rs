// Generated macro for rust_dgmm_objective (function)
macro_rules! Depcrate_saferust_dgmm_objective {
() => {
// Module: crate::safe
// Provides: {"rust_dgmm_objective"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn rust_dgmm_objective (d : usize , k : usize , n : usize , alphas : * const f64 , dalphas : * mut f64 , means : * const f64 , dmeans : * mut f64 , icf : * const f64 , dicf : * mut f64 , x : * const f64 , wishart : * const Wishart , err : * mut f64 , derr : * mut f64 ,) { let alphas = unsafe { std :: slice :: from_raw_parts (alphas , k) } ; let means = unsafe { std :: slice :: from_raw_parts (means , k * d) } ; let icf = unsafe { std :: slice :: from_raw_parts (icf , k * d * (d + 1) / 2) } ; let x = unsafe { std :: slice :: from_raw_parts (x , n * d) } ; let wishart : Wishart = unsafe { * wishart } ; let mut my_err = unsafe { * err } ; let d_alphas = unsafe { std :: slice :: from_raw_parts_mut (dalphas , k) } ; let d_means = unsafe { std :: slice :: from_raw_parts_mut (dmeans , k * d) } ; let d_icf = unsafe { std :: slice :: from_raw_parts_mut (dicf , k * d * (d + 1) / 2) } ; let mut my_derr = unsafe { * derr } ; let (mut qdiags , mut sum_qs , mut xcentered , mut qxcentered , mut main_term) = get_workspace (d , k) ; let (mut bqdiags , mut bsum_qs , mut bxcentered , mut bqxcentered , mut bmain_term) = get_workspace (d , k) ; unsafe { dgmm_objective (d , k , n , alphas , d_alphas , means , d_means , icf , d_icf , x , wishart . gamma , wishart . m , & mut my_err , & mut my_derr , & mut qdiags , & mut bqdiags , & mut sum_qs , & mut bsum_qs , & mut xcentered , & mut bxcentered , & mut qxcentered , & mut bqxcentered , & mut main_term , & mut bmain_term ,) } ; unsafe { * err = my_err } ; unsafe { * derr = my_derr } ; }
};
}
