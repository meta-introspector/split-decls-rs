// Generated macro for log_wishart_prior (function)
macro_rules! Depcrate_safelog_wishart_prior {
() => {
// Module: crate::safe
// Provides: {"log_wishart_prior"}
// Dependencies: {}
fn log_wishart_prior (p : usize , k : usize , wishart : Wishart , sum_qs : & [f64] , qdiags : & [f64] , icf : & [f64] ,) -> f64 { let n = p + wishart . m as usize + 1 ; let icf_sz = p * (p + 1) / 2 ; let c = n as f64 * p as f64 * (wishart . gamma . ln () - 0.5 * 2f64 . ln ()) - log_gamma_distrib (0.5 * n as f64 , p as f64) ; let out = (0 .. k) . map (| ik | { let frobenius = sqnorm (& qdiags [ik * p as usize ..] [.. p]) + sqnorm (& icf [ik * icf_sz as usize + p as usize ..] [.. icf_sz - p]) ; 0.5 * wishart . gamma * wishart . gamma * (frobenius) - (wishart . m as f64) * sum_qs [ik as usize] }) . sum :: < f64 > () ; out - k as f64 * c }
};
}
