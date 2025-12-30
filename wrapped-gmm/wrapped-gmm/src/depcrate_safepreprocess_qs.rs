// Generated macro for preprocess_qs (function)
macro_rules! Depcrate_safepreprocess_qs {
() => {
// Module: crate::safe
// Provides: {"preprocess_qs"}
// Dependencies: {}
fn preprocess_qs (d : usize , k : usize , icf : & [f64] , sum_qs : & mut [f64] , qdiags : & mut [f64]) { let icf_sz = d * (d + 1) / 2 ; for ik in 0 .. k { sum_qs [ik as usize] = 0. ; for id in 0 .. d { let q = icf [ik as usize * icf_sz as usize + id as usize] ; sum_qs [ik as usize] = sum_qs [ik as usize] + q ; qdiags [ik as usize * d as usize + id as usize] = q . exp () ; } } }
};
}
