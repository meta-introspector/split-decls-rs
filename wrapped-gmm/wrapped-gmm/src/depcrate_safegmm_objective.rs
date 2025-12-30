// Generated macro for gmm_objective (function)
macro_rules! Depcrate_safegmm_objective {
() => {
// Module: crate::safe
// Provides: {"gmm_objective"}
// Dependencies: {}
# [autodiff (dgmm_objective , Reverse , Const , Const , Const , Duplicated , Duplicated , Duplicated , Const , Const , Const , DuplicatedOnly , Duplicated , Duplicated , Duplicated , Duplicated , Duplicated)] pub fn gmm_objective (d : usize , k : usize , n : usize , alphas : & [f64] , means : & [f64] , icf : & [f64] , x : & [f64] , gamma : f64 , m : i32 , err : & mut f64 , qdiags : & mut [f64] , sum_qs : & mut [f64] , xcentered : & mut [f64] , qxcentered : & mut [f64] , main_term : & mut [f64] ,) { let wishart : Wishart = Wishart { gamma , m } ; let constant = - (n as f64) * d as f64 * 0.5 * (2.0 * PI) . ln () ; let icf_sz = d * (d + 1) / 2 ; assert_eq ! (qdiags . len () , d * k) ; assert_eq ! (sum_qs . len () , k) ; assert_eq ! (xcentered . len () , d) ; assert_eq ! (qxcentered . len () , d) ; assert_eq ! (main_term . len () , k) ; preprocess_qs (d , k , icf , sum_qs , qdiags) ; let mut slse = 0. ; for ix in 0 .. n { for ik in 0 .. k { subtract (d , & x [ix as usize * d as usize ..] , & means [ik as usize * d as usize ..] , xcentered ,) ; qtimesx (d , & qdiags [ik as usize * d as usize ..] , & icf [ik as usize * icf_sz as usize + d as usize ..] , & * xcentered , qxcentered ,) ; main_term [ik as usize] = alphas [ik as usize] + sum_qs [ik as usize] - 0.5 * sqnorm (& * qxcentered) ; } slse = slse + log_sum_exp (k , & main_term) ; } let lse_alphas = log_sum_exp (k , alphas) ; * err = constant + slse - n as f64 * lse_alphas + log_wishart_prior (d , k , wishart , & sum_qs , & * qdiags , icf) ; }
};
}
